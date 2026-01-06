use crate::cyrene::{Fragment, Function, Instruction, Label, Terminator};
use crate::demiurge::meta::{Lattice, Meta};
use std::collections::HashSet;

enum Writeback {
    All(Label),
    Once(Label),
    None,
}

impl Function {
    pub fn rewrite(&mut self, meta: &Meta) -> bool {
        let mut changed = self.prune(meta);

        let mut writebacks = Vec::new();
        for (label, fragment) in self.cautious() {
            let mut wb = Writeback::None;
            if fragment.rewrite(meta, &mut wb) {
                changed = true;
            }
            writebacks.push((wb, label));
        }

        for (wb, delete) in writebacks {
            match wb {
                Writeback::All(from) => {
                    let Some(frag) = self.modify(from) else {
                        continue;
                    };
                    frag.predecessors.retain(|x| *x != delete);
                    for (_, inputs) in frag.phis.iter_mut() {
                        inputs.retain(|(x, _)| *x != delete);
                    }
                }
                Writeback::Once(from) => {
                    let Some(frag) = self.modify(from) else {
                        continue;
                    };
                    if let Some(idx) = frag.predecessors.iter().position(|x| *x == delete) {
                        frag.predecessors.remove(idx);
                    }
                    for (_, inputs) in frag.phis.iter_mut() {
                        if let Some(idx) = inputs.iter().position(|(x, _)| *x == delete) {
                            inputs.remove(idx);
                        }
                    }
                }
                Writeback::None => {}
            }
        }

        changed
    }

    fn prune(&mut self, meta: &Meta) -> bool {
        let mut eliminated = HashSet::new();
        self.fragments.retain(|id, _| {
            let label = Label::Id(*id);
            let keep = meta.visited.contains(&label);
            if !keep {
                eliminated.insert(label);
            }
            keep
        });

        if eliminated.is_empty() {
            return false;
        }

        for (_, fragment) in self.cautious() {
            fragment
                .predecessors
                .retain(|label| !eliminated.contains(label));
            fragment
                .phis
                .iter_mut()
                .for_each(|(_, inputs)| inputs.retain(|(label, _)| !eliminated.contains(label)));
        }
        true
    }
}

impl Fragment {
    fn rewrite(&mut self, meta: &Meta, wb: &mut Writeback) -> bool {
        let mut changed = false;
        let mut new = Vec::new();
        self.phis.retain(|(x, _)| {
            if let Lattice::Const(c) = meta.get(*x) {
                new.push(Instruction::Load(*x, c.clone()));
                changed = true;
                return false;
            }
            true
        });
        for instruction in self.instructions.iter_mut() {
            if instruction.rewrite(meta) {
                changed = true;
            }
        }
        self.instructions.splice(0..0, new);
        if self.terminator.as_mut().unwrap().rewrite(meta, wb) {
            changed = true;
        }
        changed
    }
}

impl Instruction {
    fn rewrite(&mut self, meta: &Meta) -> bool {
        match self {
            Instruction::Binary(dst, _, _, _) | Instruction::Unary(dst, _, _) => {
                if let Lattice::Const(c) = meta.get(*dst) {
                    *self = Instruction::Load(*dst, c.clone());
                    return true;
                }
            }
            _ => {}
        }
        false
    }
}

impl Terminator {
    fn rewrite(&mut self, meta: &Meta, wb: &mut Writeback) -> bool {
        if let Terminator::Branch(cond, yes, no) = self {
            if let Lattice::Const(c) = meta.get(*cond)
                && let Ok(b) = c.bool()
            {
                let (target, dead) = if b { (yes, no) } else { (no, yes) };
                *wb = if target == dead {
                    Writeback::Once(*dead)
                } else {
                    Writeback::All(*dead)
                };
                *self = Terminator::Jump(*target);
                return true;
            } else if yes == no {
                *wb = Writeback::Once(*no);
                *self = Terminator::Jump(*yes);
                return true;
            }
        }
        false
    }
}

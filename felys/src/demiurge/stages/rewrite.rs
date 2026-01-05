use crate::cyrene::{Fragment, Function, Instruction, Label, Terminator};
use crate::demiurge::meta::{Lattice, Meta};

impl Function {
    pub fn rewrite(&mut self, meta: &Meta) -> bool {
        let mut changed = false;

        let mut writebacks = Vec::new();
        for (label, fragment) in self.dangerous() {
            let mut wb = None;
            if fragment.rewrite(meta, &mut wb) {
                changed = true;
            }
            if let Some(wb) = wb {
                writebacks.push((wb, label));
            }
        }

        for (from, delete) in writebacks {
            let Some(frag) = self.modify(from) else {
                continue;
            };
            frag.predecessors.retain(|x| *x != delete);
            for (_, inputs) in frag.phis.iter_mut() {
                inputs.retain(|(x, _)| *x != delete);
            }
        }

        changed
    }
}

impl Fragment {
    fn rewrite(&mut self, meta: &Meta, wb: &mut Option<Label>) -> bool {
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
    fn rewrite(&mut self, meta: &Meta, wb: &mut Option<Label>) -> bool {
        if let Terminator::Branch(cond, yes, no) = self {
            if let Lattice::Const(c) = meta.get(*cond) {
                let (target, dead) = if c.bool().unwrap() {
                    (yes, no)
                } else {
                    (no, yes)
                };
                if target != dead {
                    *wb = Some(*dead)
                }
                *self = Terminator::Jump(*target);
                return true;
            } else if yes == no {
                *self = Terminator::Jump(*yes);
                return true;
            }
        }
        false
    }
}

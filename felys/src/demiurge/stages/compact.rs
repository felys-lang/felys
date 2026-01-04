use crate::cyrene::{Label, Terminator};
use crate::demiurge::Function;

impl Function {
    pub fn compact(&mut self) -> bool {
        let mut changed = false;

        while let Some((id, target)) = self.redundant() {
            let empty = Label::Id(id);
            let predecessors = self.get(empty).unwrap().predecessors.clone();

            for pred in predecessors.iter() {
                let frag = self.modify(*pred).unwrap();
                match frag.terminator.as_mut().unwrap() {
                    Terminator::Jump(t) => {
                        if *t == empty {
                            *t = target;
                        }
                    }
                    Terminator::Branch(_, yes, no) => {
                        if *yes == empty {
                            *yes = target;
                        }
                        if *no == empty {
                            *no = target;
                        }
                    }
                    _ => {}
                }
            }

            let frag = self.modify(target).unwrap();
            frag.predecessors.retain(|x| *x != empty);
            frag.predecessors.extend_from_slice(&predecessors);

            for (_, inputs) in frag.phis.iter_mut() {
                let Some((_, var)) = inputs.iter().find(|(x, _)| *x == empty) else {
                    continue;
                };
                let var = *var;
                inputs.retain(|(x, _)| *x != empty);
                for pred in predecessors.iter() {
                    inputs.push((*pred, var));
                }
            }

            self.fragments.remove(&id);
            changed = true;
        }
        changed
    }

    fn redundant(&self) -> Option<(usize, Label)> {
        let mut candidate = None;
        for (id, frag) in self.fragments.iter() {
            if frag.instructions.is_empty()
                && frag.phis.is_empty()
                && let Terminator::Jump(target) = frag.terminator.as_ref().unwrap()
                && *target != Label::Id(*id)
            {
                candidate = Some((*id, *target));
            }
        }
        candidate
    }
}

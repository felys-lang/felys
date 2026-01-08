use crate::cyrene::{Fragment, Function, Label, Terminator};
use std::collections::VecDeque;

impl Function {
    pub fn compact(&mut self) -> bool {
        let mut changed = false;
        let mut worklist = VecDeque::new();
        for (label, _) in self.safe() {
            worklist.push_back(label);
        }

        while let Some(empty) = worklist.pop_front() {
            let fragment = self.get(empty).unwrap();
            let Some(target) = fragment.mergeable(empty) else {
                continue;
            };
            changed = true;
            let pred = fragment.predecessors[0];

            let predecessor = self.modify(pred).unwrap();
            match predecessor.terminator.as_mut().unwrap() {
                Terminator::Branch(_, _, _) => {}
                Terminator::Jump(x) => *x = target,
                Terminator::Return(_) => {}
            }

            let successor = self.modify(target).unwrap();
            successor.predecessors.iter_mut().for_each(|label| {
                if label == &empty {
                    *label = pred
                }
            });
            successor.phis.iter_mut().for_each(|(_, inputs)| {
                inputs.iter_mut().for_each(|(label, _)| {
                    if label == &empty {
                        *label = pred
                    }
                })
            });

            worklist.push_back(pred);
            worklist.push_back(target);
        }

        changed
    }
}

impl Fragment {
    fn mergeable(&self, label: Label) -> Option<Label> {
        if self.instructions.is_empty()
            && self.phis.is_empty()
            && self.predecessors.len() == 1
            && let Terminator::Jump(target) = self.terminator.as_ref().unwrap()
            && *target != label
        {
            Some(*target)
        } else {
            None
        }
    }
}

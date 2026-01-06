use crate::cyrene::{Function, Label, Terminator, Var};
use std::collections::HashMap;

impl Function {
    pub fn copies(&mut self) -> HashMap<Label, Vec<(Var, Var)>> {
        self.split();

        let mut copies = HashMap::new();
        for (_, fragment) in self.safe() {
            for (dst, inputs) in fragment.phis.iter() {
                for (from, src) in inputs {
                    copies
                        .entry(*from)
                        .or_insert_with(Vec::new)
                        .push((*dst, *src));
                }
            }
        }

        copies
            .iter_mut()
            .for_each(|(_, pending)| *pending = self.decycle(pending));

        copies
    }

    fn decycle(&mut self, pending: &mut Vec<(Var, Var)>) -> Vec<(Var, Var)> {
        pending.retain(|(dst, src)| dst != src);
        let mut copies = Vec::new();

        while !pending.is_empty() {
            let ready = pending
                .iter()
                .position(|&(dst, _)| !pending.iter().any(|&(_, src)| src == dst));

            if let Some(idx) = ready {
                let copy = pending.swap_remove(idx);
                copies.push(copy);
                continue;
            }

            let (_, breakpoint) = pending[0];
            let temp = self.var();
            copies.push((temp, breakpoint));

            for (_, src) in pending.iter_mut() {
                if *src == breakpoint {
                    *src = temp;
                }
            }
        }

        copies
    }

    fn split(&mut self) {
        let mut edges = Vec::new();
        for (label, fragment) in self.safe() {
            let Terminator::Branch(_, yes, no) = fragment.terminator.as_ref().unwrap() else {
                continue;
            };
            for target in [*yes, *no] {
                let frag = self.get(target).unwrap();
                if frag.predecessors.len() > 1 {
                    edges.push((label, target));
                }
            }
        }

        for (label, target) in edges {
            let trampoline = self.label();

            let fragment = self.add(trampoline);
            fragment.predecessors.push(label);
            fragment.terminator = Some(Terminator::Jump(target));

            let fragment = self.modify(label).unwrap();
            match fragment.terminator.as_mut().unwrap() {
                Terminator::Branch(_, yes, no) => {
                    if *yes == target {
                        *yes = trampoline;
                    } else if *no == target {
                        *no = trampoline;
                    } else {
                        panic!()
                    }
                }
                _ => panic!(),
            }

            let fragment = self.modify(target).unwrap();
            *fragment
                .predecessors
                .iter_mut()
                .find(|x| **x == label)
                .unwrap() = trampoline;
            for (_, inputs) in fragment.phis.iter_mut() {
                let (x, _) = inputs.iter_mut().find(|(x, _)| *x == label).unwrap();
                *x = trampoline;
            }
        }
    }
}

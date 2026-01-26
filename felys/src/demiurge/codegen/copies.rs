use crate::utils::function::Function;
use crate::utils::ir::{Label, Terminator, Var};
use std::collections::HashMap;

pub struct Copy(pub Var, pub Var);

impl Function {
    pub fn copies(&mut self) -> HashMap<Label, Vec<Copy>> {
        self.split();

        let mut copies = HashMap::new();
        for (_, fragment) in self.safe() {
            for phi in fragment.phis.iter() {
                for (from, src) in phi.inputs.iter() {
                    copies
                        .entry(*from)
                        .or_insert_with(Vec::new)
                        .push(Copy(phi.var, *src));
                }
            }
        }

        copies
            .iter_mut()
            .for_each(|(_, pending)| self.decycle(pending));

        copies
    }

    fn decycle(&mut self, pending: &mut Vec<Copy>) {
        pending.retain(|Copy(dst, src)| dst != src);
        let mut copies = Vec::new();

        while !pending.is_empty() {
            let ready = pending
                .iter()
                .position(|Copy(dst, _)| !pending.iter().any(|Copy(_, src)| src == dst));

            if let Some(idx) = ready {
                let copy = pending.swap_remove(idx);
                copies.push(copy);
                continue;
            }

            let Copy(_, breakpoint) = pending[0];
            let temp = self.var();
            copies.push(Copy(temp, breakpoint));

            for Copy(_, src) in pending.iter_mut() {
                if *src == breakpoint {
                    *src = temp;
                }
            }
        }

        *pending = copies
    }

    fn split(&mut self) {
        let mut edges = Vec::new();
        for (label, fragment) in self.safe() {
            let Some(Terminator::Branch(_, yes, no)) = fragment.terminator.as_ref() else {
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

            let fragment = self.modify(label).unwrap();
            match fragment.terminator.as_mut().unwrap() {
                Terminator::Branch(_, yes, no) => {
                    if *yes == target {
                        *yes = trampoline;
                    } else if *no == target {
                        *no = trampoline;
                    } else {
                        continue;
                    }
                }
                _ => continue,
            }

            let fragment = self.modify(trampoline).unwrap();
            fragment.predecessors.push(label);
            fragment.terminator = Some(Terminator::Jump(target));

            let fragment = self.modify(target).unwrap();
            *fragment
                .predecessors
                .iter_mut()
                .find(|x| **x == label)
                .unwrap() = trampoline;
            for phi in fragment.phis.iter_mut() {
                let (x, _) = phi.inputs.iter_mut().find(|(x, _)| *x == label).unwrap();
                *x = trampoline;
            }
        }
    }
}

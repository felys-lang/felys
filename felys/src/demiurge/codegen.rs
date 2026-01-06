use crate::cyrene::{Function, Terminator};
use crate::demiurge::Demiurge;

impl Demiurge {
    pub fn split(&mut self) {
        for function in self.fns.values_mut() {
            function.split();
        }
        self.main.split();
    }
}

impl Function {
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
            fragment.predecessors.iter_mut().for_each(|pred| {
                if *pred == label {
                    *pred = trampoline;
                }
            });
            for (_, inputs) in fragment.phis.iter_mut() {
                inputs.iter_mut().for_each(|(pred, _)| {
                    if *pred == label {
                        *pred = trampoline;
                    }
                });
            }
        }
    }
}

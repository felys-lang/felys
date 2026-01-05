use crate::cyrene::{Function, Terminator};
use crate::demiurge::Demiurge;

impl Demiurge {
    pub fn split(&self) {
        for function in self.fns.values() {
            function.split();
        }
        self.main.split();
    }
}

impl Function {
    fn split(&self) {
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
    }
}

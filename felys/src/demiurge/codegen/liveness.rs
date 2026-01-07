use crate::cyrene::{Fragment, Function, Instruction, Label, Terminator, Var};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

type Liveness = HashMap<Label, HashSet<Var>>;

impl Function {
    fn liveness(&self, copies: &HashMap<Label, Vec<(Var, Var)>>) -> (Liveness, Liveness) {
        let mut ins = Liveness::new();
        let mut outs = Liveness::new();
        let fixed = self
            .safe()
            .map(|(label, fragment)| (label, fragment.precompute(copies.get(&label).unwrap())))
            .collect::<HashMap<_, _>>();

        let mut worklist = VecDeque::new();
        let mut queued = HashSet::new();
        for (label, _) in self.safe() {
            worklist.push_back(label);
            queued.insert(label);
        }

        while let Some(label) = worklist.pop_front() {
            queued.remove(&label);

            let (uses, defs) = fixed.get(&label).unwrap();
            let fragment = self.get(label).unwrap();
            let mut set = HashSet::new();
            for succ in fragment.successors() {
                let i = ins.get(&succ).unwrap_or(uses);
                set.extend(i.iter());
            }

            match outs.entry(label) {
                Entry::Occupied(e) if e.get() == &set => continue,
                e => {
                    e.insert_entry(set.clone());
                }
            }

            set.retain(|x| !defs.contains(x));
            set.extend(uses.iter());
            ins.insert(label, set);

            for pred in fragment.predecessors.iter() {
                if queued.insert(*pred) {
                    worklist.push_back(*pred);
                }
            }
        }
        (ins, outs)
    }
}

impl Fragment {
    fn precompute(&self, copies: &[(Var, Var)]) -> (HashSet<Var>, HashSet<Var>) {
        let mut uses = HashSet::new();
        let mut defs = HashSet::new();
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Field(dst, src, _) | Instruction::Unary(dst, _, src) => {
                    if !defs.contains(src) {
                        uses.insert(*src);
                    }
                    defs.insert(*dst);
                }
                Instruction::Group(dst, _)
                | Instruction::Function(dst, _)
                | Instruction::Load(dst, _) => {
                    defs.insert(*dst);
                }
                Instruction::Binary(dst, lhs, _, rhs) | Instruction::Index(dst, lhs, rhs) => {
                    if !defs.contains(lhs) {
                        uses.insert(*lhs);
                    }
                    if !defs.contains(rhs) {
                        uses.insert(*rhs);
                    }
                    defs.insert(*dst);
                }
                Instruction::Call(dst, src, args) | Instruction::Method(dst, src, _, args) => {
                    if !defs.contains(src) {
                        uses.insert(*src);
                    }
                    for arg in args {
                        if !defs.contains(arg) {
                            uses.insert(*arg);
                        }
                    }
                    defs.insert(*dst);
                }
                Instruction::List(dst, args) | Instruction::Tuple(dst, args) => {
                    for arg in args {
                        if !defs.contains(arg) {
                            uses.insert(*arg);
                        }
                    }
                    defs.insert(*dst);
                }
            }
        }
        for (dst, src) in copies.iter() {
            if !defs.contains(src) {
                uses.insert(*src);
            }
            defs.insert(*dst);
        }
        match self.terminator.as_ref().unwrap() {
            Terminator::Branch(var, _, _) | Terminator::Return(var) => {
                if !defs.contains(var) {
                    uses.insert(*var);
                }
            }
            Terminator::Jump(_) => {}
        }
        (uses, defs)
    }
}

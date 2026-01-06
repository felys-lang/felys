use crate::cyrene::{Fragment, Function, Instruction, Terminator, Var};
use std::collections::HashMap;

struct Renamer {
    map: HashMap<Var, Var>,
}

impl Renamer {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, from: Var, to: Var) {
        self.map.insert(from, to);
    }

    fn get(&self, var: Var, changed: &mut bool) -> Var {
        let mut current = var;
        while let Some(&next) = self.map.get(&current) {
            current = next;
        }
        if current != var {
            *changed = true;
        }
        current
    }
}

impl Function {
    pub fn rename(&mut self) -> bool {
        let mut renamer = Renamer::new();
        let mut changed = false;

        let mut again = true;
        while again {
            again = false;
            for (_, fragment) in self.dangerous() {
                if fragment.simplify(&mut renamer, &mut changed) {
                    again = true;
                }
            }
        }

        if renamer.map.is_empty() {
            return changed;
        }

        for (_, fragment) in self.dangerous() {
            if fragment.rename(&renamer) {
                changed = true;
            }
        }
        changed
    }
}

impl Fragment {
    fn simplify(&mut self, renamer: &mut Renamer, changed: &mut bool) -> bool {
        let mut again = false;
        self.phis.retain(|(var, input)| {
            let mut trivial = true;
            let mut candidate = None;
            for (_, src) in input {
                let resolved = renamer.get(*src, &mut false);
                if resolved == *var {
                    continue;
                }
                if let Some(c) = candidate {
                    if c != resolved {
                        trivial = false;
                        break;
                    }
                } else {
                    candidate = Some(resolved);
                }
            }

            if trivial && let Some(replacement) = candidate {
                renamer.insert(*var, replacement);
                *changed = true;
                again = true;
                false
            } else {
                true
            }
        });
        again
    }

    fn rename(&mut self, renamer: &Renamer) -> bool {
        let mut changed = false;
        for (_, inputs) in self.phis.iter_mut() {
            for (_, var) in inputs {
                *var = renamer.get(*var, &mut changed);
            }
        }

        for instruction in self.instructions.iter_mut() {
            if instruction.rename(renamer) {
                changed = true;
            }
        }
        if self.terminator.as_mut().unwrap().rename(renamer) {
            changed = true;
        }
        changed
    }
}

impl Instruction {
    fn rename(&mut self, renamer: &Renamer) -> bool {
        let mut changed = false;
        match self {
            Instruction::Binary(_, lhs, _, rhs) => {
                *lhs = renamer.get(*lhs, &mut changed);
                *rhs = renamer.get(*rhs, &mut changed);
            }
            Instruction::Unary(_, _, var) | Instruction::Field(_, var, _) => {
                *var = renamer.get(*var, &mut changed)
            }
            Instruction::Call(_, var, params)
            | Instruction::Method(_, var, _, params)
            | Instruction::List(var, params)
            | Instruction::Tuple(var, params) => {
                *var = renamer.get(*var, &mut changed);
                params
                    .iter_mut()
                    .for_each(|x| *x = renamer.get(*x, &mut changed));
            }
            Instruction::Index(_, var, index) => {
                *var = renamer.get(*var, &mut changed);
                *index = renamer.get(*index, &mut changed);
            }
            _ => {}
        }
        changed
    }
}

impl Terminator {
    fn rename(&mut self, renamer: &Renamer) -> bool {
        let mut changed = false;
        match self {
            Terminator::Branch(var, _, _) | Terminator::Return(var) => {
                *var = renamer.get(*var, &mut changed)
            }
            _ => {}
        }
        changed
    }
}

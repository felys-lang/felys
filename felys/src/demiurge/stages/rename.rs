use crate::cyrene::{Fragment, Instruction, Label, Terminator, Var};
use crate::demiurge::meta::Meta;
use crate::demiurge::Function;
use std::collections::{HashMap, HashSet};

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
        let mut visited = HashSet::new();
        while let Some(&next) = self.map.get(&current) {
            if !visited.insert(next) {
                break;
            }
            current = next;
        }
        if current != var {
            *changed = true;
        }
        current
    }
}

impl Function {
    pub fn rename(&mut self, meta: &Meta) -> bool {
        let mut renamer = Renamer::new();
        let mut changed = false;

        let mut again = true;
        while again {
            again = false;
            for (label, fragment) in self.dangerous() {
                if fragment.simplify(label, meta, &mut renamer, &mut changed) {
                    again = true;
                }
            }
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
    fn simplify(
        &mut self,
        label: Label,
        meta: &Meta,
        renamer: &mut Renamer,
        changed: &mut bool,
    ) -> bool {
        let mut again = false;
        for (_, inputs) in self.phis.iter_mut() {
            let len = inputs.len();
            inputs.retain(|(pred, _)| {
                let keep = meta.edges.contains(&(*pred, label));
                if !keep {
                    *changed = true;
                }
                keep
            });
            if len != inputs.len() {
                again = true;
            }
        }

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

            let keep = if trivial && let Some(replacement) = candidate {
                renamer.insert(*var, replacement);
                again = true;
                false
            } else {
                true
            };
            if !keep {
                *changed = true;
            }
            keep
        });
        again
    }

    fn rename(&mut self, renamer: &Renamer) -> bool {
        let mut changed = false;
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

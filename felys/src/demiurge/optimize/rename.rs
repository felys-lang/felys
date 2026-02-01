use crate::utils::function::{Fragment, Function, Phi};
use crate::utils::ir::{Instruction, Terminator, Var};
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

    fn get(&self, var: Var) -> Var {
        let mut current = var;
        while let Some(&next) = self.map.get(&current) {
            current = next;
        }
        current
    }
}

impl Function {
    pub fn rename(&mut self) -> bool {
        let mut renamer = Renamer::new();

        let mut again = true;
        while again {
            again = false;
            for (_, fragment) in self.cautious() {
                fragment.phis.retain(|phi| {
                    let mut trivial = true;
                    let mut candidate = None;
                    for (_, src) in phi.inputs.iter() {
                        let resolved = renamer.get(*src);
                        if resolved == phi.var {
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
                        renamer.insert(phi.var, replacement);
                        again = true;
                        false
                    } else {
                        true
                    }
                });
            }
        }

        if renamer.map.is_empty() {
            return false;
        }

        for (_, fragment) in self.cautious() {
            fragment.rename(&renamer);
        }
        true
    }
}

impl Fragment {
    fn rename(&mut self, renamer: &Renamer) {
        for phi in self.phis.iter_mut() {
            phi.rename(renamer);
        }
        for instruction in self.instructions.iter_mut() {
            instruction.rename(renamer);
        }
        self.terminator.as_mut().unwrap().rename(renamer)
    }
}

impl Phi {
    fn rename(&mut self, renamer: &Renamer) {
        for (_, var) in self.inputs.iter_mut() {
            *var = renamer.get(*var);
        }
    }
}

impl Instruction {
    fn rename(&mut self, renamer: &Renamer) {
        match self {
            Instruction::Unary(_, _, src)
            | Instruction::Field(_, src, _)
            | Instruction::Unpack(_, src, _) => *src = renamer.get(*src),
            Instruction::Binary(_, src, _, other) | Instruction::Index(_, src, other) => {
                *src = renamer.get(*src);
                *other = renamer.get(*other);
            }
            Instruction::List(_, args) | Instruction::Tuple(_, args) => {
                args.iter_mut().for_each(|x| *x = renamer.get(*x));
            }
            Instruction::Call(_, src, args) | Instruction::Method(_, src, _, args) => {
                *src = renamer.get(*src);
                args.iter_mut().for_each(|x| *x = renamer.get(*x));
            }
            Instruction::Arg(_, _) | Instruction::Pointer(_, _, _) | Instruction::Load(_, _) => {}
        }
    }
}

impl Terminator {
    fn rename(&mut self, renamer: &Renamer) {
        match self {
            Terminator::Branch(var, _, _) | Terminator::Return(var) => *var = renamer.get(*var),
            Terminator::Jump(_) => {}
        }
    }
}

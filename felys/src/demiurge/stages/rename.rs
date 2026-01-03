use crate::cyrene::{Fragment, Instruction, Label, Terminator};
use crate::demiurge::context::{Meta, Renamer};
use crate::demiurge::Function;

impl Function {
    pub fn rename(&mut self, ctx: &Meta) {
        let mut renamer = Renamer::new();
        let mut changed = true;
        while changed {
            changed = false;
            for (label, fragment) in self.dangerous() {
                if fragment.simplify(label, ctx, &mut renamer) {
                    changed = true;
                }
            }
        }

        for (_, fragment) in self.dangerous() {
            fragment.rename(&renamer);
        }
    }
}

impl Fragment {
    fn simplify(&mut self, label: Label, ctx: &Meta, renamer: &mut Renamer) -> bool {
        let mut changed = false;
        for (_, inputs) in self.phis.iter_mut() {
            let len = inputs.len();
            inputs.retain(|(pred, _)| ctx.edges.contains(&(*pred, label)));
            if len != inputs.len() {
                changed = true;
            }
        }

        self.phis.retain(|(var, input)| {
            let mut trivial = true;
            let mut candidate = None;
            for (_, src) in input {
                let resolved = renamer.get(*src);
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
                changed = true;
                return false;
            }
            true
        });
        changed
    }

    fn rename(&mut self, renamer: &Renamer) {
        for instruction in self.instructions.iter_mut() {
            instruction.rename(renamer);
        }
        self.terminator.as_mut().unwrap().rename(renamer);
    }
}

impl Instruction {
    fn rename(&mut self, renamer: &Renamer) {
        match self {
            Instruction::Binary(_, lhs, _, rhs) => {
                *lhs = renamer.get(*lhs);
                *rhs = renamer.get(*rhs);
            }
            Instruction::Unary(_, _, var) | Instruction::Field(_, var, _) => {
                *var = renamer.get(*var)
            }
            Instruction::Call(_, var, params)
            | Instruction::Method(_, var, _, params)
            | Instruction::List(var, params)
            | Instruction::Tuple(var, params) => {
                *var = renamer.get(*var);
                params.iter_mut().for_each(|x| *x = renamer.get(*x));
            }
            Instruction::Index(_, var, index) => {
                *var = renamer.get(*var);
                *index = renamer.get(*index);
            }
            _ => {}
        }
    }
}

impl Terminator {
    fn rename(&mut self, renamer: &Renamer) {
        match self {
            Terminator::Branch(var, _, _) | Terminator::Return(var) => *var = renamer.get(*var),
            _ => {}
        }
    }
}

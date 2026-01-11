use crate::cyrene::{Fragment, Function, Instruction, Label, Terminator, Var};
use crate::demiurge::fault::Fault;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Default)]
struct Context {
    active: HashSet<Var>,
    keep: HashMap<Label, HashSet<usize>>,
    worklist: VecDeque<Var>,
    defs: HashMap<Var, (Label, Id)>,
}

enum Id {
    Ins(usize),
    Phi,
    Arg,
}

impl Function {
    pub fn sweep(&mut self) -> Result<bool, Fault> {
        let mut ctx = Context::default();
        for arg in self.args.clone() {
            ctx.defs.insert(arg, (Label::Entry, Id::Arg));
        }

        for (label, fragment) in self.safe() {
            fragment.initialize(label, &mut ctx);
        }

        while let Some(var) = ctx.worklist.pop_front() {
            let (label, id) = ctx.defs.get(&var).ok_or(Fault::ValueUnreachable)?;
            let fragment = self.get(*label).unwrap();

            match id {
                Id::Ins(idx) => {
                    if ctx.keep.entry(*label).or_default().insert(*idx) {
                        let instruction = fragment.instructions.get(*idx).unwrap();
                        instruction.visit(&mut ctx);
                    }
                }
                Id::Phi => {
                    let (_, inputs) = fragment.phis.iter().find(|(v, _)| *v == var).unwrap();
                    for (_, input) in inputs {
                        if ctx.active.insert(*input) {
                            ctx.worklist.push_back(*input);
                        }
                    }
                }
                _ => {}
            }
        }

        let mut changed = false;
        for (label, fragment) in self.cautious() {
            if fragment.sweep(label, &ctx) {
                changed = true;
            }
        }
        Ok(changed)
    }
}

impl Fragment {
    fn sweep(&mut self, label: Label, ctx: &Context) -> bool {
        let mut changed = false;
        if let Some(indices) = ctx.keep.get(&label) {
            let mut i = 0;
            self.instructions.retain(|_| {
                let keep = indices.contains(&i);
                if !keep {
                    changed = true
                }
                i += 1;
                keep
            });
        } else if !self.instructions.is_empty() {
            self.instructions.clear();
            changed = true;
        }

        self.phis.retain(|(var, _)| {
            let keep = ctx.active.contains(var);
            if !keep {
                changed = true
            }
            keep
        });
        changed
    }

    fn initialize(&self, label: Label, ctx: &mut Context) {
        for (var, _) in self.phis.iter() {
            ctx.defs.insert(*var, (label, Id::Phi));
        }

        for (idx, instruction) in self.instructions.iter().enumerate() {
            ctx.defs.insert(instruction.dst(), (label, Id::Ins(idx)));

            if !instruction.functional() {
                ctx.keep.entry(label).or_default().insert(idx);
                instruction.visit(ctx);
            }
        }
        self.terminator.as_ref().unwrap().visit(ctx);
    }
}

impl Instruction {
    fn visit(&self, ctx: &mut Context) {
        let mut add = |var: &Var| {
            if ctx.active.insert(*var) {
                ctx.worklist.push_back(*var);
            }
        };
        match self {
            Instruction::Field(_, var, _) | Instruction::Unary(_, _, var) => add(var),
            Instruction::List(_, params) | Instruction::Tuple(_, params) => {
                params.iter().for_each(add);
            }
            Instruction::Binary(_, lhs, _, rhs) | Instruction::Index(_, lhs, rhs) => {
                add(lhs);
                add(rhs);
            }
            Instruction::Call(_, var, params) | Instruction::Method(_, var, _, params) => {
                add(var);
                params.iter().for_each(add);
            }
            _ => {}
        }
    }

    fn dst(&self) -> Var {
        match self {
            Instruction::Field(dst, _, _)
            | Instruction::Function(dst, _)
            | Instruction::Load(dst, _)
            | Instruction::Binary(dst, _, _, _)
            | Instruction::Unary(dst, _, _)
            | Instruction::Call(dst, _, _)
            | Instruction::List(dst, _)
            | Instruction::Tuple(dst, _)
            | Instruction::Index(dst, _, _)
            | Instruction::Method(dst, _, _, _)
            | Instruction::Group(dst, _) => *dst,
        }
    }

    fn functional(&self) -> bool {
        !matches!(self, Instruction::Call(..) | Instruction::Method(..))
    }
}

impl Terminator {
    fn visit(&self, ctx: &mut Context) {
        let mut add = |var: &Var| {
            if ctx.active.insert(*var) {
                ctx.worklist.push_back(*var);
            }
        };
        match self {
            Terminator::Branch(var, _, _) | Terminator::Return(var) => add(var),
            _ => {}
        }
    }
}

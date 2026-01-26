use crate::utils::function::{Fragment, Function, Phi};
use crate::utils::ir::{Instruction, Label, Terminator, Var};
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
    Phi(usize),
}

impl Function {
    pub fn sweep(&mut self) -> bool {
        let mut ctx = Context::default();

        for (label, fragment) in self.safe() {
            fragment.initialize(label, &mut ctx);
        }

        while let Some(var) = ctx.worklist.pop_front() {
            let (label, id) = ctx.defs.get(&var).unwrap();
            let fragment = self.get(*label).unwrap();

            match id {
                Id::Ins(index) => {
                    if ctx.keep.entry(*label).or_default().insert(*index) {
                        let instruction = fragment.instructions.get(*index).unwrap();
                        instruction.visit(&mut ctx);
                    }
                }
                Id::Phi(index) => {
                    let phi = fragment.phis.get(*index).unwrap();
                    phi.visit(&mut ctx);
                }
            }
        }

        let mut changed = false;
        for (label, fragment) in self.cautious() {
            if fragment.sweep(label, &ctx) {
                changed = true;
            }
        }
        changed
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

        self.phis.retain(|phi| {
            let keep = ctx.active.contains(&phi.var);
            if !keep {
                changed = true
            }
            keep
        });
        changed
    }

    fn initialize(&self, label: Label, ctx: &mut Context) {
        for (i, phi) in self.phis.iter().enumerate() {
            ctx.defs.insert(phi.var, (label, Id::Phi(i)));
        }

        for (i, instruction) in self.instructions.iter().enumerate() {
            ctx.defs.insert(instruction.dst(), (label, Id::Ins(i)));

            if !instruction.functional() {
                ctx.keep.entry(label).or_default().insert(i);
                instruction.visit(ctx);
            }
        }
        self.terminator.as_ref().unwrap().visit(ctx);
    }
}

impl Phi {
    fn visit(&self, ctx: &mut Context) {
        for (_, input) in self.inputs.iter() {
            if ctx.active.insert(*input) {
                ctx.worklist.push_back(*input);
            }
        }
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
            Instruction::Field(_, src, _)
            | Instruction::Unpack(_, src, _)
            | Instruction::Unary(_, _, src) => add(src),
            Instruction::Binary(_, src, _, other) | Instruction::Index(_, src, other) => {
                add(src);
                add(other);
            }
            Instruction::List(_, args) | Instruction::Tuple(_, args) => {
                args.iter().for_each(add);
            }
            Instruction::Call(_, src, args) | Instruction::Method(_, src, _, args) => {
                add(src);
                args.iter().for_each(add);
            }
            Instruction::Arg(_, _) | Instruction::Pointer(_, _, _) | Instruction::Load(_, _) => {}
        }
    }

    fn dst(&self) -> Var {
        match self {
            Instruction::Arg(dst, _)
            | Instruction::Field(dst, _, _)
            | Instruction::Unpack(dst, _, _)
            | Instruction::Load(dst, _)
            | Instruction::Binary(dst, _, _, _)
            | Instruction::Unary(dst, _, _)
            | Instruction::Call(dst, _, _)
            | Instruction::List(dst, _)
            | Instruction::Tuple(dst, _)
            | Instruction::Index(dst, _, _)
            | Instruction::Method(dst, _, _, _)
            | Instruction::Pointer(dst, _, _) => *dst,
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
            Terminator::Jump(_) => {}
        }
    }
}

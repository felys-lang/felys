use crate::cyrene::{Function, Instruction, Label, Terminator, Var};
use crate::demiurge::codegen::copies::Copy;
use crate::demiurge::Reg;
use std::cmp::{max, Reverse};
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Id {
    Ins(usize),
    Copy(usize),
    Term,
}

#[derive(Default, Debug)]
struct Context {
    indices: HashMap<(Label, Id), usize>,
    defs: HashMap<Var, usize>,
    uses: HashMap<Var, usize>,
}

impl Function {
    pub fn allocate(&self, copies: &HashMap<Label, Vec<Copy>>) -> HashMap<Var, Reg> {
        let ctx = self.precompute(copies);
        let mut intervals = ctx
            .uses
            .iter()
            .map(|(var, last)| (*var, *ctx.defs.get(var).unwrap(), *last))
            .collect::<Vec<_>>();
        intervals.sort_by_key(|(_, start, _)| *start);

        let mut active = BinaryHeap::<Reverse<(Var, Reg)>>::new();
        let mut used = 0;
        let mut registers = Vec::new();
        let mut mapping = HashMap::new();

        for (var, start, end) in intervals {
            while let Some(Reverse((e, reg))) = active.peek().cloned() {
                if e <= start {
                    active.pop();
                    registers.push(reg);
                } else {
                    break;
                }
            }

            let reg = registers.pop().unwrap_or_else(|| {
                used += 1;
                used
            });
            mapping.insert(var, reg);
            active.push(Reverse((end, reg)));
        }

        mapping
    }

    fn precompute(&self, copies: &HashMap<Label, Vec<Copy>>) -> Context {
        let mut ctx = Context::default();

        let mut index = 0;
        for i in 0..self.args.len() {
            ctx.defs.insert(i, index);
            index += 1;
        }
        for label in self.rpo() {
            let fragment = self.get(label).unwrap();
            for (idx, instruction) in fragment.instructions.iter().enumerate() {
                instruction.update(index, &mut ctx);
                ctx.indices.insert((label, Id::Ins(idx)), index);
                index += 1;
            }
            if let Some(copy) = copies.get(&label) {
                for (idx, copy) in copy.iter().enumerate() {
                    copy.uses(index, &mut ctx);
                    ctx.indices.insert((label, Id::Copy(idx)), index);
                    index += 1;
                }
            }
            fragment.terminator.as_ref().unwrap().uses(index, &mut ctx);
            ctx.indices.insert((label, Id::Term), index);
            index += 1;
        }

        ctx
    }
}

impl Instruction {
    fn update(&self, index: usize, ctx: &mut Context) {
        let mut update = |var: &Var| {
            match ctx.uses.entry(*var) {
                Entry::Occupied(mut e) => {
                    let last = max(index, *e.get());
                    e.insert(last);
                }
                Entry::Vacant(e) => {
                    e.insert(index);
                }
            };
        };
        match self {
            Instruction::Field(dst, src, _) | Instruction::Unary(dst, _, src) => {
                ctx.defs.insert(*dst, index);
                update(src)
            }
            Instruction::Binary(dst, lhs, _, rhs) | Instruction::Index(dst, lhs, rhs) => {
                ctx.defs.insert(*dst, index);
                update(lhs);
                update(rhs);
            }
            Instruction::Call(dst, src, args) | Instruction::Method(dst, src, _, args) => {
                ctx.defs.insert(*dst, index);
                update(src);
                for arg in args {
                    update(arg);
                }
            }
            Instruction::List(dst, args) | Instruction::Tuple(dst, args) => {
                ctx.defs.insert(*dst, index);
                for arg in args {
                    update(arg);
                }
            }
            Instruction::Group(dst, _)
            | Instruction::Function(dst, _)
            | Instruction::Load(dst, _) => {
                ctx.defs.insert(*dst, index);
            }
        }
    }
}

impl Terminator {
    fn uses(&self, index: usize, ctx: &mut Context) {
        let mut update = |var: &Var| {
            match ctx.uses.entry(*var) {
                Entry::Occupied(mut e) => {
                    let last = max(index, *e.get());
                    e.insert(last);
                }
                Entry::Vacant(e) => {
                    e.insert(index);
                }
            };
        };
        match self {
            Terminator::Branch(var, _, _) | Terminator::Return(var) => {
                update(var);
            }
            Terminator::Jump(_) => {}
        }
    }
}

impl Copy {
    fn uses(&self, index: usize, ctx: &mut Context) {
        let mut update = |var: &Var| {
            match ctx.uses.entry(*var) {
                Entry::Occupied(mut e) => {
                    let last = max(index, *e.get());
                    e.insert(last);
                }
                Entry::Vacant(e) => {
                    e.insert(index);
                }
            };
        };
        ctx.defs.insert(self.0, index);
        update(&self.1);
    }
}

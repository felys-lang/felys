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

impl Context {
    fn define(&mut self, var: &Var, index: usize) {
        self.defs.entry(*var).or_insert(index);
    }

    fn using(&mut self, var: &Var, index: usize) {
        match self.uses.entry(*var) {
            Entry::Occupied(mut e) => {
                let last = max(index, *e.get());
                e.insert(last);
            }
            Entry::Vacant(e) => {
                e.insert(index);
            }
        };
    }
}

impl Function {
    pub fn allocate(&self, copies: &HashMap<Label, Vec<Copy>>) -> (HashMap<Var, Reg>, usize) {
        let ctx = self.precompute(copies);
        let mut intervals = ctx
            .uses
            .iter()
            .map(|(var, last)| (*var, ctx.defs[var], *last))
            .collect::<Vec<_>>();
        intervals.sort_by_key(|(_, start, _)| *start);

        let mut active = BinaryHeap::<Reverse<(Var, Reg)>>::new();
        let mut used = 1;
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
                let new = used;
                used += 1;
                new
            });
            mapping.insert(var, reg);
            active.push(Reverse((end, reg)));
        }

        (mapping, used - 1)
    }

    fn precompute(&self, copies: &HashMap<Label, Vec<Copy>>) -> Context {
        let mut ctx = Context::default();
        let mut anchors = HashMap::new();
        let mut loops = Vec::new();

        let mut index = 0;
        for i in self.args.clone() {
            ctx.defs.insert(i, index);
            index += 1;
        }
        for label in self.rpo() {
            anchors.insert(label, index);
            let fragment = self.get(label).unwrap();
            for (idx, instruction) in fragment.instructions.iter().enumerate() {
                instruction.du(index, &mut ctx);
                ctx.indices.insert((label, Id::Ins(idx)), index);
                index += 1;
            }
            if let Some(copy) = copies.get(&label) {
                for (idx, copy) in copy.iter().enumerate() {
                    copy.du(index, &mut ctx);
                    ctx.indices.insert((label, Id::Copy(idx)), index);
                    index += 1;
                }
            }
            fragment
                .terminator
                .as_ref()
                .unwrap()
                .du(index, &mut ctx, &mut anchors, &mut loops);
            ctx.indices.insert((label, Id::Term), index);

            index += 1;
        }

        for (start, end) in loops {
            for (var, last) in ctx.uses.iter_mut() {
                let def = ctx.defs[var];
                if def < start && *last >= start {
                    *last = max(*last, end);
                }
            }
        }

        ctx
    }
}

impl Instruction {
    fn du(&self, index: usize, ctx: &mut Context) {
        match self {
            Instruction::Field(dst, src, _)
            | Instruction::Unpack(dst, src, _)
            | Instruction::Unary(dst, _, src) => {
                ctx.define(dst, index);
                ctx.using(src, index);
            }

            Instruction::Binary(dst, lhs, _, rhs) | Instruction::Index(dst, lhs, rhs) => {
                ctx.define(dst, index);
                ctx.using(lhs, index);
                ctx.using(rhs, index);
            }
            Instruction::Call(dst, src, args) | Instruction::Method(dst, src, _, args) => {
                ctx.define(dst, index);
                ctx.using(src, index);
                for arg in args {
                    ctx.using(arg, index);
                }
            }
            Instruction::List(dst, args) | Instruction::Tuple(dst, args) => {
                ctx.define(dst, index);
                for arg in args {
                    ctx.using(arg, index);
                }
            }
            Instruction::Group(dst, _)
            | Instruction::Function(dst, _)
            | Instruction::Load(dst, _) => {
                ctx.define(dst, index);
            }
        }
    }
}

impl Terminator {
    fn du(
        &self,
        index: usize,
        ctx: &mut Context,
        anchors: &mut HashMap<Label, usize>,
        loops: &mut Vec<(usize, usize)>,
    ) {
        let mut extend = |label: &Label| {
            if let Some(start) = anchors.get(label) {
                loops.push((*start, index));
            }
        };

        match self {
            Terminator::Branch(var, yes, no) => {
                ctx.using(var, index);
                extend(yes);
                extend(no);
            }
            Terminator::Return(var) => {
                ctx.using(var, index);
            }
            Terminator::Jump(target) => {
                extend(target);
            }
        }
    }
}

impl Copy {
    fn du(&self, index: usize, ctx: &mut Context) {
        ctx.define(&self.0, index);
        ctx.using(&self.1, index);
    }
}

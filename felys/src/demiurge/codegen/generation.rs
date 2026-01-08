use crate::cyrene::{Const, Function, Instruction, Label, Terminator};
use crate::demiurge::codegen::copies::Copy;
use crate::demiurge::{Bytecode, Demiurge};
use crate::elysia::Elysia;
use std::collections::HashMap;
use std::hash::Hash;

struct Context {
    consts: Pooling<Const>,
    groups: Pooling<usize>,
    functions: Pooling<usize>,
}

#[derive(Default)]
struct Pooling<T> {
    pool: Vec<T>,
    fast: HashMap<T, usize>,
}

impl<T: Hash + Eq + Clone> Pooling<T> {
    fn idx(&mut self, key: T) -> usize {
        if let Some(&id) = self.fast.get(&key) {
            return id;
        }
        let id = self.pool.len();
        self.fast.insert(key.clone(), id);
        self.pool.push(key);
        id
    }
}

impl Demiurge {
    pub fn codegen(mut self) -> Elysia {
        let mut ctx = Context {
            consts: Pooling {
                pool: Vec::new(),
                fast: HashMap::new(),
            },
            groups: Default::default(),
            functions: Default::default(),
        };
        let main = self.main.codegen(&mut ctx);
        let text = self
            .fns
            .into_values()
            .map(|mut x| x.codegen(&mut ctx))
            .collect();
        let lookup = ctx
            .groups
            .pool
            .into_iter()
            .map(|x| self.groups.remove(&x).unwrap())
            .collect();
        Elysia {
            main,
            text,
            data: ctx.consts.pool,
            lookup,
        }
    }
}

impl Function {
    fn codegen(&mut self, ctx: &mut Context) -> Vec<Bytecode> {
        let copies = self.copies();
        let allocation = self.allocate(&copies);
        // println!("{:?}", allocation);
        self.lowering(ctx, copies)
    }

    fn lowering(
        &mut self,
        ctx: &mut Context,
        mut copies: HashMap<Label, Vec<Copy>>,
    ) -> Vec<Bytecode> {
        let mut index = 0;
        let mut map = HashMap::new();
        for label in self.order() {
            map.insert(label, index);
            let fragment = self.get(label).unwrap();
            let body = fragment.instructions.len();
            let copy = copies.get(&label).map(|x| x.len()).unwrap_or(0);
            index += copy + body + 1
        }

        let mut bytecodes = Vec::with_capacity(index);
        for label in self.order() {
            let fragment = self.get(label).unwrap();
            let copy = copies
                .remove(&label)
                .unwrap_or_default()
                .into_iter()
                .map(|copy| copy.codegen());
            let body = fragment.instructions.iter().map(|x| x.codegen(ctx));
            let goto = fragment.terminator.as_ref().unwrap().codegen(&map);
            bytecodes.extend(body);
            bytecodes.extend(copy);
            bytecodes.push(goto);
        }
        bytecodes
    }
}

impl Instruction {
    fn codegen(&self, ctx: &mut Context) -> Bytecode {
        match self {
            Instruction::Field(dst, src, idx) => Bytecode::Field(*dst, *src, *idx),
            Instruction::Group(dst, id) => Bytecode::Group(*dst, ctx.groups.idx(*id)),
            Instruction::Function(dst, id) => Bytecode::Function(*dst, ctx.functions.idx(*id)),
            Instruction::Load(dst, id) => Bytecode::Load(*dst, ctx.consts.idx(id.clone())),
            Instruction::Binary(dst, lhs, op, rhs) => {
                Bytecode::Binary(*dst, *lhs, op.clone(), *rhs)
            }
            Instruction::Unary(dst, op, src) => Bytecode::Unary(*dst, op.clone(), *src),
            Instruction::Call(dst, src, args) => Bytecode::Call(*dst, *src, args.clone()),
            Instruction::List(dst, args) => Bytecode::List(*dst, args.clone()),
            Instruction::Tuple(dst, args) => Bytecode::Tuple(*dst, args.clone()),
            Instruction::Index(dst, src, index) => Bytecode::Index(*dst, *src, *index),
            Instruction::Method(dst, src, id, args) => {
                Bytecode::Method(*dst, *src, *id, args.clone())
            }
        }
    }
}

impl Terminator {
    fn codegen(&self, map: &HashMap<Label, usize>) -> Bytecode {
        match self {
            Terminator::Branch(cond, yes, no) => {
                Bytecode::Branch(*cond, *map.get(yes).unwrap(), *map.get(no).unwrap())
            }
            Terminator::Jump(to) => Bytecode::Jump(*map.get(to).unwrap()),
            Terminator::Return(var) => Bytecode::Return(*var),
        }
    }
}

impl Copy {
    fn codegen(&self) -> Bytecode {
        Bytecode::Copy(self.0, self.1)
    }
}

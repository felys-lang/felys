use crate::cyrene::{Const, Function, Instruction, Label, Terminator, Var};
use crate::demiurge::codegen::copies::Copy;
use crate::demiurge::{Bytecode, Demiurge, Reg};
use crate::elysia::{Callable, Elysia};
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
        let mut functions = self
            .fns
            .into_iter()
            .map(|(label, mut x)| (label, x.codegen(&mut ctx)))
            .collect::<HashMap<_, _>>();

        let mut groups = Vec::new();
        for id in ctx.groups.pool {
            let mut group = self.groups.remove(&id).unwrap();
            group
                .methods
                .iter_mut()
                .for_each(|(_, x)| *x = ctx.functions.idx(*x));
            groups.push(group);
        }

        let mut text = Vec::new();
        for id in ctx.functions.pool {
            let function = functions.remove(&id).unwrap();
            text.push(function)
        }

        Elysia {
            main,
            text,
            data: ctx.consts.pool,
            groups,
        }
    }
}

impl Function {
    fn codegen(&mut self, ctx: &mut Context) -> Callable {
        let copies = self.copies();
        let (allocation, used) = self.allocate(&copies);
        Callable {
            args: self.args.end,
            registers: used,
            bytecodes: self.lowering(ctx, &allocation, copies),
        }
    }

    fn lowering(
        &mut self,
        ctx: &mut Context,
        alloc: &HashMap<Var, Reg>,
        mut copies: HashMap<Label, Vec<Copy>>,
    ) -> Vec<Bytecode> {
        let mut index = 0;
        let mut map = HashMap::new();
        for label in self.order() {
            map.insert(label, index);
            let fragment = self.get(label).unwrap();
            let body = fragment.instructions.len();
            let copy = copies.get(&label).map(|x| x.len()).unwrap_or(0);
            let term = fragment.terminator.is_some() as usize;
            index += copy + body + term
        }

        let mut bytecodes = Vec::with_capacity(index);
        for label in self.order() {
            let fragment = self.get(label).unwrap();
            let body = fragment.instructions.iter().map(|x| x.codegen(alloc, ctx));
            let copy = copies
                .remove(&label)
                .unwrap_or_default()
                .into_iter()
                .map(|copy| copy.codegen(alloc));
            let term = fragment.terminator.as_ref().map(|x| x.codegen(alloc, &map));
            bytecodes.extend(body);
            bytecodes.extend(copy);
            if let Some(term) = term {
                bytecodes.push(term);
            }
        }
        bytecodes
    }
}

impl Instruction {
    fn codegen(&self, alloc: &HashMap<Var, Reg>, ctx: &mut Context) -> Bytecode {
        match self {
            Instruction::Field(dst, src, idx) => Bytecode::Field(
                *alloc.get(dst).unwrap_or(&0),
                *alloc.get(src).unwrap_or(&0),
                *idx,
            ),
            Instruction::Unpack(dst, src, idx) => Bytecode::Unpack(
                *alloc.get(dst).unwrap_or(&0),
                *alloc.get(src).unwrap_or(&0),
                *idx,
            ),
            Instruction::Group(dst, id) => {
                Bytecode::Group(*alloc.get(dst).unwrap_or(&0), ctx.groups.idx(*id))
            }
            Instruction::Function(dst, id) => {
                Bytecode::Function(*alloc.get(dst).unwrap_or(&0), ctx.functions.idx(*id))
            }
            Instruction::Load(dst, id) => {
                Bytecode::Load(*alloc.get(dst).unwrap_or(&0), ctx.consts.idx(id.clone()))
            }
            Instruction::Binary(dst, lhs, op, rhs) => Bytecode::Binary(
                *alloc.get(dst).unwrap_or(&0),
                *alloc.get(lhs).unwrap_or(&0),
                op.clone(),
                *alloc.get(rhs).unwrap_or(&0),
            ),
            Instruction::Unary(dst, op, src) => Bytecode::Unary(
                *alloc.get(dst).unwrap_or(&0),
                op.clone(),
                *alloc.get(src).unwrap_or(&0),
            ),
            Instruction::Call(dst, src, args) => Bytecode::Call(
                *alloc.get(dst).unwrap_or(&0),
                *alloc.get(src).unwrap_or(&0),
                args.iter().map(|x| *alloc.get(x).unwrap_or(&0)).collect(),
            ),
            Instruction::List(dst, args) => Bytecode::List(
                *alloc.get(dst).unwrap_or(&0),
                args.iter().map(|x| *alloc.get(x).unwrap_or(&0)).collect(),
            ),
            Instruction::Tuple(dst, args) => Bytecode::Tuple(
                *alloc.get(dst).unwrap_or(&0),
                args.iter().map(|x| *alloc.get(x).unwrap_or(&0)).collect(),
            ),
            Instruction::Index(dst, src, index) => Bytecode::Index(
                *alloc.get(dst).unwrap_or(&0),
                *alloc.get(src).unwrap_or(&0),
                *alloc.get(index).unwrap_or(&0),
            ),
            Instruction::Method(dst, src, id, args) => Bytecode::Method(
                *alloc.get(dst).unwrap_or(&0),
                *alloc.get(src).unwrap_or(&0),
                *id,
                args.iter().map(|x| *alloc.get(x).unwrap_or(&0)).collect(),
            ),
        }
    }
}

impl Terminator {
    fn codegen(&self, alloc: &HashMap<Var, Reg>, map: &HashMap<Label, usize>) -> Bytecode {
        match self {
            Terminator::Branch(cond, yes, no) => {
                Bytecode::Branch(*alloc.get(cond).unwrap_or(&0), map[yes], map[no])
            }
            Terminator::Jump(to) => Bytecode::Jump(map[to]),
            Terminator::Return(var) => Bytecode::Return(*alloc.get(var).unwrap_or(&0)),
        }
    }
}

impl Copy {
    fn codegen(&self, alloc: &HashMap<Var, Reg>) -> Bytecode {
        Bytecode::Copy(
            *alloc.get(&self.0).unwrap_or(&0),
            *alloc.get(&self.1).unwrap_or(&0),
        )
    }
}

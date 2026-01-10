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
            index += copy + body + 1
        }

        let mut bytecodes = Vec::with_capacity(index);
        for label in self.order() {
            let fragment = self.get(label).unwrap();
            let copy = copies
                .remove(&label)
                .unwrap_or_default()
                .into_iter()
                .filter_map(|copy| copy.codegen(alloc));
            let body = fragment
                .instructions
                .iter()
                .filter_map(|x| x.codegen(alloc, ctx));
            let goto = fragment.terminator.as_ref().unwrap().codegen(alloc, &map);
            bytecodes.extend(body);
            bytecodes.extend(copy);
            bytecodes.push(goto);
        }
        bytecodes
    }
}

impl Instruction {
    fn codegen(&self, alloc: &HashMap<Var, Reg>, ctx: &mut Context) -> Option<Bytecode> {
        let bytecode = match self {
            Instruction::Field(dst, src, idx) => {
                Bytecode::Field(*alloc.get(dst)?, alloc[src], *idx)
            }
            Instruction::Group(dst, id) => Bytecode::Group(*alloc.get(dst)?, ctx.groups.idx(*id)),
            Instruction::Function(dst, id) => {
                Bytecode::Function(*alloc.get(dst)?, ctx.functions.idx(*id))
            }
            Instruction::Load(dst, id) => {
                Bytecode::Load(*alloc.get(dst)?, ctx.consts.idx(id.clone()))
            }
            Instruction::Binary(dst, lhs, op, rhs) => {
                Bytecode::Binary(*alloc.get(dst)?, alloc[lhs], op.clone(), alloc[rhs])
            }
            Instruction::Unary(dst, op, src) => {
                Bytecode::Unary(*alloc.get(dst)?, op.clone(), alloc[src])
            }
            Instruction::Call(dst, src, args) => {
                Bytecode::Call(*alloc.get(dst)?, alloc[src], args.clone())
            }
            Instruction::List(dst, args) => {
                Bytecode::List(*alloc.get(dst)?, args.iter().map(|x| alloc[x]).collect())
            }
            Instruction::Tuple(dst, args) => Bytecode::Tuple(
                *alloc.get(dst)?,
                args.iter()
                    .map(|x| alloc.get(x).cloned())
                    .collect::<Option<_>>()?,
            ),
            Instruction::Index(dst, src, index) => {
                Bytecode::Index(*alloc.get(dst)?, alloc[src], alloc[index])
            }
            Instruction::Method(dst, src, id, args) => Bytecode::Method(
                *alloc.get(dst)?,
                alloc[src],
                *id,
                args.iter().map(|x| alloc[x]).collect(),
            ),
        };
        Some(bytecode)
    }
}

impl Terminator {
    fn codegen(&self, alloc: &HashMap<Var, Reg>, map: &HashMap<Label, usize>) -> Bytecode {
        match self {
            Terminator::Branch(cond, yes, no) => Bytecode::Branch(alloc[cond], map[yes], map[no]),
            Terminator::Jump(to) => Bytecode::Jump(map[to]),
            Terminator::Return(var) => Bytecode::Return(alloc[var]),
        }
    }
}

impl Copy {
    fn codegen(&self, alloc: &HashMap<Var, Reg>) -> Option<Bytecode> {
        let bytecode = Bytecode::Copy(*alloc.get(&self.0)?, alloc[&self.1]);
        Some(bytecode)
    }
}

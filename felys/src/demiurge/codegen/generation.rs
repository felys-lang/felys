use crate::demiurge::codegen::copies::Copy;
use crate::utils::bytecode::{Bytecode, Reg};
use crate::utils::function::Function;
use crate::utils::group::Group;
use crate::utils::ir::{Const, Instruction, Label, Pointer, Terminator, Var};
use crate::utils::stages::{Callable, Demiurge, Elysia};
use crate::utils::stdlib::utils::stdlib;
use std::collections::HashMap;
use std::hash::Hash;

struct Context {
    consts: Pooling<Const>,
    groups: Worker<Group, Group>,
    functions: Worker<Callable, Function>,
}

struct Pooling<T> {
    pool: Vec<T>,
    fast: HashMap<T, usize>,
}

impl<T: Hash + Eq + Clone> Pooling<T> {
    fn index(&mut self, key: T) -> usize {
        if let Some(&id) = self.fast.get(&key) {
            return id;
        }
        let id = self.pool.len();
        self.fast.insert(key.clone(), id);
        self.pool.push(key);
        id
    }
}

struct Worker<T, S> {
    pool: HashMap<usize, T>,
    indices: HashMap<usize, usize>,
    source: HashMap<usize, S>,
}

impl<T, S> Worker<T, S> {
    fn linearize(mut self) -> Vec<T> {
        let mut i = 0;
        let mut all = Vec::new();
        while let Some(value) = self.pool.remove(&i) {
            all.push(value);
            i += 1;
        }
        all
    }
}

impl Context {
    fn new(gps: HashMap<usize, Group>, fns: HashMap<usize, Function>) -> Self {
        Self {
            consts: Pooling {
                pool: Vec::new(),
                fast: HashMap::new(),
            },
            groups: Worker {
                pool: HashMap::new(),
                indices: HashMap::new(),
                source: gps,
            },
            functions: Worker {
                pool: HashMap::new(),
                indices: HashMap::new(),
                source: fns,
            },
        }
    }

    fn group(&mut self, key: usize) -> usize {
        if let Some(index) = self.groups.indices.get(&key) {
            *index
        } else {
            let index = self.groups.indices.len();
            self.groups.indices.insert(key, index);
            let mut group = self.groups.source.remove(&key).unwrap();
            group
                .methods
                .values_mut()
                .for_each(|x| *x = self.function(*x));
            self.groups.pool.insert(index, group);
            index
        }
    }

    fn function(&mut self, key: usize) -> usize {
        if let Some(index) = self.functions.indices.get(&key) {
            *index
        } else {
            let index = self.functions.indices.len();
            self.functions.indices.insert(key, index);
            let mut function = self.functions.source.remove(&key).unwrap();
            let callable = function.codegen(self);
            self.functions.pool.insert(index, callable);
            index
        }
    }
}

impl Demiurge {
    pub fn codegen(mut self) -> Elysia {
        let mut ctx = Context::new(self.gps, self.fns);
        Elysia {
            main: self.main.codegen(&mut ctx),
            text: ctx.functions.linearize(),
            rust: stdlib().map(|(_, _, _, x)| x).collect(),
            data: ctx.consts.pool,
            groups: ctx.groups.linearize(),
        }
    }
}

impl Function {
    fn codegen(&mut self, ctx: &mut Context) -> Callable {
        let copies = self.copies();
        let (allocation, used) = self.allocate(&copies);
        Callable {
            args: self.args,
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
            Instruction::Arg(dst, idx) => Bytecode::Arg(alloc[dst], *idx),
            Instruction::Field(dst, src, idx) => Bytecode::Field(alloc[dst], alloc[src], *idx),
            Instruction::Unpack(dst, src, idx) => Bytecode::Unpack(alloc[dst], alloc[src], *idx),
            Instruction::Pointer(dst, pt, ptr) => match pt {
                Pointer::Function => Bytecode::Pointer(alloc[dst], pt.clone(), ctx.function(*ptr)),
                Pointer::Group => Bytecode::Pointer(alloc[dst], pt.clone(), ctx.group(*ptr)),
                Pointer::Rust => Bytecode::Pointer(alloc[dst], pt.clone(), *ptr),
            },
            Instruction::Load(dst, id) => Bytecode::Load(alloc[dst], ctx.consts.index(id.clone())),
            Instruction::Binary(dst, lhs, op, rhs) => Bytecode::Binary(
                alloc[dst],
                *alloc.get(lhs).unwrap_or(&0),
                op.clone(),
                *alloc.get(rhs).unwrap_or(&0),
            ),
            Instruction::Unary(dst, op, src) => Bytecode::Unary(alloc[dst], op.clone(), alloc[src]),
            Instruction::Call(dst, src, args) => Bytecode::Call(
                alloc[dst],
                alloc[src],
                args.iter().map(|x| alloc[x]).collect(),
            ),
            Instruction::List(dst, args) => {
                Bytecode::List(alloc[dst], args.iter().map(|x| alloc[x]).collect())
            }
            Instruction::Tuple(dst, args) => {
                Bytecode::Tuple(alloc[dst], args.iter().map(|x| alloc[x]).collect())
            }
            Instruction::Index(dst, src, index) => {
                Bytecode::Index(alloc[dst], alloc[src], alloc[index])
            }
            Instruction::Method(dst, src, id, args) => Bytecode::Method(
                alloc[dst],
                alloc[src],
                *id,
                args.iter().map(|x| alloc[x]).collect(),
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

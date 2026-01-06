use crate::cyrene::{Const, Function, Instruction, Label, Terminator, Var};
use crate::demiurge::{Bytecode, Demiurge};
use crate::elysia::Elysia;
use std::collections::HashMap;
use std::hash::Hash;

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

impl Function {
    fn codegen(&mut self, ctx: &mut Context) -> Vec<Bytecode> {
        self.split();
        let copies = self.copies();
        self.destruct(ctx, copies)
    }

    fn destruct(
        &mut self,
        ctx: &mut Context,
        mut copies: HashMap<Label, Vec<(Var, Var)>>,
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
                .map(|(dst, src)| Bytecode::Copy(dst, src));
            let body = fragment.instructions.iter().map(|x| x.codegen(ctx));
            let goto = fragment.terminator.as_ref().unwrap().codegen(&map);
            bytecodes.extend(body);
            bytecodes.extend(copy);
            bytecodes.push(goto);
        }
        bytecodes
    }

    fn copies(&mut self) -> HashMap<Label, Vec<(Var, Var)>> {
        let mut copies = HashMap::new();
        for (_, fragment) in self.safe() {
            for (dst, inputs) in fragment.phis.iter() {
                for (from, src) in inputs {
                    copies
                        .entry(*from)
                        .or_insert_with(Vec::new)
                        .push((*dst, *src));
                }
            }
        }

        copies
            .iter_mut()
            .for_each(|(_, pending)| *pending = self.decycle(pending));

        copies
    }

    fn decycle(&mut self, pending: &mut Vec<(Var, Var)>) -> Vec<(Var, Var)> {
        pending.retain(|(dst, src)| dst != src);
        let mut copies = Vec::new();

        while !pending.is_empty() {
            let ready = pending
                .iter()
                .position(|&(dst, _)| !pending.iter().any(|&(_, src)| src == dst));

            if let Some(idx) = ready {
                let copy = pending.swap_remove(idx);
                copies.push(copy);
                continue;
            }

            let (_, breakpoint) = pending[0];
            let temp = self.var();
            copies.push((temp, breakpoint));

            for (_, src) in pending.iter_mut() {
                if *src == breakpoint {
                    *src = temp;
                }
            }
        }

        copies
    }

    fn split(&mut self) {
        let mut edges = Vec::new();
        for (label, fragment) in self.safe() {
            let Terminator::Branch(_, yes, no) = fragment.terminator.as_ref().unwrap() else {
                continue;
            };
            for target in [*yes, *no] {
                let frag = self.get(target).unwrap();
                if frag.predecessors.len() > 1 {
                    edges.push((label, target));
                }
            }
        }

        for (label, target) in edges {
            let trampoline = self.label();

            let fragment = self.add(trampoline);
            fragment.predecessors.push(label);
            fragment.terminator = Some(Terminator::Jump(target));

            let fragment = self.modify(label).unwrap();
            match fragment.terminator.as_mut().unwrap() {
                Terminator::Branch(_, yes, no) => {
                    if *yes == target {
                        *yes = trampoline;
                    } else if *no == target {
                        *no = trampoline;
                    } else {
                        panic!()
                    }
                }
                _ => panic!(),
            }

            let fragment = self.modify(target).unwrap();
            *fragment
                .predecessors
                .iter_mut()
                .find(|x| **x == label)
                .unwrap() = trampoline;
            for (_, inputs) in fragment.phis.iter_mut() {
                let (x, _) = inputs.iter_mut().find(|(x, _)| *x == label).unwrap();
                *x = trampoline;
            }
        }
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

use crate::cyrene::{Const, Function, Group, Instruction};
use crate::demiurge::bytecode::Bytecode;
use crate::elysia::Elysia;
use crate::error::Fault;
use crate::philia093::Intern;
use std::collections::HashMap;

pub struct Demiurge {
    pub groups: HashMap<usize, Group>,
    pub fns: HashMap<usize, Function>,
    pub main: Function,
    pub intern: Intern,
}
// 
// struct Data {
//     data: Vec<Const>,
//     fast: HashMap<Const, usize>,
// }
// 
// impl Data {
//     fn idx(&mut self, c: Const) -> usize {
//         if let Some(idx) = self.fast.get(&c) {
//             *idx
//         } else {
//             let idx = self.data.len();
//             self.fast.insert(c.clone(), idx);
//             self.data.push(c);
//             idx
//         }
//     }
// }
// 
// impl Demiurge {
//     pub fn codegen(mut self) -> Result<Elysia, Fault> {
//         let mut data = Data {
//             data: Vec::new(),
//             fast: HashMap::new(),
//         };
// 
//         let mut text = Vec::new();
//         let mut fid2idx = HashMap::new();
//         let mut todo = Vec::new();
//         for group in self.groups.values() {
//             for id in group.methods.values() {
//                 todo.push(*id);
//             }
//         }
//         let mut main = self.main.codegen(&mut data, &mut todo)?;
// 
//         while let Some(next) = todo.pop() {
//             if fid2idx.contains_key(&next) {
//                 continue;
//             }
//             let f = self.fns.remove(&next).unwrap();
//             let func = f.codegen(&mut data, &mut todo)?;
//             fid2idx.insert(next, text.len());
//             text.push(func);
//         }
// 
//         let mut lookup = Vec::new();
//         let mut gid2idx = HashMap::new();
// 
//         for bytecode in main.iter_mut() {
//             bytecode.flush(&fid2idx, &mut gid2idx, &mut self.groups, &mut lookup)
//         }
//         for function in text.iter_mut() {
//             for bytecode in function.iter_mut() {
//                 bytecode.flush(&fid2idx, &mut gid2idx, &mut self.groups, &mut lookup)
//             }
//         }
// 
//         Ok(Elysia {
//             main,
//             text,
//             data: data.data,
//             lookup,
//         })
//     }
// }
// 
// impl Function {
//     fn codegen(self, data: &mut Data, todo: &mut Vec<usize>) -> Result<Vec<Bytecode>, Fault> {
//         let mut idx = 0;
//         let mut labels = HashMap::new();
//         for segment in self.fragments.iter() {
//             labels.insert(segment.label, idx);
//             idx += segment.instructions.len();
//         }
// 
//         let mut bytecodes = Vec::new();
//         for segment in self.fragments {
//             for instruction in segment.instructions {
//                 let bytecode = match instruction {
//                     Instruction::Field(dst, src, id) => Bytecode::Field(dst, src, id),
//                     Instruction::Func(dst, id) => {
//                         todo.push(id);
//                         Bytecode::Func(dst, id)
//                     }
//                     Instruction::Load(dst, c) => {
//                         let idx = data.idx(c);
//                         Bytecode::Load(dst, idx)
//                     }
//                     Instruction::Binary(dst, l, op, r) => Bytecode::Binary(dst, l, op, r),
//                     Instruction::Unary(dst, op, inner) => Bytecode::Unary(dst, op, inner),
//                     Instruction::Copy(dst, src) => Bytecode::Copy(dst, src),
//                     Instruction::Branch(cond, on, label) => {
//                         let idx = labels.get(&label).cloned().unwrap();
//                         Bytecode::Branch(cond, on, idx)
//                     }
//                     Instruction::Jump(label) => {
//                         let idx = labels.get(&label).cloned().unwrap();
//                         Bytecode::Jump(idx)
//                     }
//                     Instruction::Return(value) => Bytecode::Return(value),
//                     Instruction::Buffer => Bytecode::Buffer,
//                     Instruction::Push(var) => Bytecode::Push(var),
//                     Instruction::Call(dst, src) => Bytecode::Call(dst, src),
//                     Instruction::List(dst) => Bytecode::List(dst),
//                     Instruction::Tuple(dst) => Bytecode::Tuple(dst),
//                     Instruction::Index(dst, src, index) => Bytecode::Index(dst, src, index),
//                     Instruction::Method(dst, src, id) => Bytecode::Method(dst, src, id),
//                     Instruction::Group(dst, id) => Bytecode::Group(dst, id),
//                 };
//                 bytecodes.push(bytecode);
//             }
//         }
//         Ok(bytecodes)
//     }
// }
// 
// impl Bytecode {
//     fn flush(
//         &mut self,
//         fid2idx: &HashMap<usize, usize>,
//         gid2idx: &mut HashMap<usize, usize>,
//         groups: &mut HashMap<usize, Group>,
//         lookup: &mut Vec<Group>,
//     ) {
//         match self {
//             Bytecode::Func(_, id) => {
//                 *id = fid2idx.get(id).cloned().unwrap();
//             }
//             Bytecode::Group(_, id) => {
//                 if let Some(idx) = gid2idx.get(id) {
//                     *id = *idx;
//                 } else {
//                     let idx = lookup.len();
//                     gid2idx.insert(*id, idx);
//                     let group = groups.remove(id).unwrap();
//                     lookup.push(group);
//                     *id = idx;
//                 }
//             }
//             _ => {}
//         }
//     }
// }

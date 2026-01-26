use crate::demiurge::Bytecode;
use crate::utils::group::Group;
use crate::utils::ir::Const;
use crate::utils::stdlib::utils::Signature;
use std::io::Write;

pub struct Elysia {
    pub main: Callable,
    pub text: Vec<Callable>,
    pub rust: Vec<Signature>,
    pub data: Vec<Const>,
    pub groups: Vec<Group>,
}

#[derive(Debug)]
pub struct Callable {
    pub args: usize,
    pub registers: usize,
    pub bytecodes: Vec<Bytecode>,
}

impl Elysia {
    pub fn dump<W: Write>(&self, buf: &mut W) -> std::io::Result<()> {
        self.main.dump(buf)?;
        for callable in self.text.iter() {
            callable.dump(buf)?;
        }
        Ok(())
    }
}

impl Callable {
    fn dump<W: Write>(&self, buf: &mut W) -> std::io::Result<()> {
        buf.write_all(&[self.args as u8, self.registers as u8])?;
        buf.write_all(&(self.bytecodes.len() as u32).to_le_bytes())?;
        for bytecode in self.bytecodes.iter() {
            bytecode.dump(buf)?;
        }
        Ok(())
    }
}

impl Bytecode {
    fn dump<W: Write>(&self, buf: &mut W) -> std::io::Result<()> {
        match self {
            Bytecode::Arg(dst, idx) => {
                buf.write_all(&[0x0, *dst as u8, *idx as u8])?;
            }
            Bytecode::Field(dst, src, id) => {
                buf.write_all(&[0x1, *dst as u8, *src as u8])?;
                buf.write_all(&(*id as u32).to_le_bytes())?;
            }
            Bytecode::Unpack(dst, src, idx) => {
                buf.write_all(&[0x2, *dst as u8, *src as u8])?;
                buf.write_all(&(*idx as u32).to_le_bytes())?;
            }
            Bytecode::Pointer(dst, pt, ptr) => {
                buf.write_all(&[0x3, *dst as u8, pt.clone() as u8, *ptr as u8])?;
            }
            Bytecode::Load(dst, idx) => {
                buf.write_all(&[0x4, *dst as u8, *idx as u8])?;
            }
            Bytecode::Binary(dst, lhs, op, rhs) => {
                buf.write_all(&[0x5, *dst as u8, *lhs as u8, op.clone() as u8, *rhs as u8])?;
            }
            Bytecode::Unary(dst, op, src) => {
                buf.write_all(&[0x6, *dst as u8, op.clone() as u8, *src as u8])?;
            }
            Bytecode::Call(dst, src, args) => {
                buf.write_all(&[0x7, *dst as u8, *src as u8, args.len() as u8])?;
                buf.write_all(&args.iter().map(|&x| x as u8).collect::<Vec<_>>())?;
            }
            Bytecode::List(dst, args) => {
                buf.write_all(&[0x8, *dst as u8, args.len() as u8])?;
                buf.write_all(&args.iter().map(|&x| x as u8).collect::<Vec<_>>())?;
            }
            Bytecode::Tuple(dst, args) => {
                buf.write_all(&[0x9, *dst as u8, args.len() as u8])?;
                buf.write_all(&args.iter().map(|&x| x as u8).collect::<Vec<_>>())?;
            }
            Bytecode::Index(dst, src, index) => {
                buf.write_all(&[0xA, *dst as u8, *src as u8, *index as u8])?;
            }
            Bytecode::Method(dst, src, id, args) => {
                buf.write_all(&[0xB, *dst as u8, *src as u8])?;
                buf.write_all(&(*id as u32).to_le_bytes())?;
                buf.write_all(&args.iter().map(|&x| x as u8).collect::<Vec<_>>())?;
            }
            Bytecode::Branch(cond, yes, no) => {
                buf.write_all(&[0xC, *cond as u8])?;
                buf.write_all(&(*yes as u32).to_le_bytes())?;
                buf.write_all(&(*no as u32).to_le_bytes())?;
            }
            Bytecode::Jump(target) => {
                buf.write_all(&[0xD])?;
                buf.write_all(&(*target as u32).to_le_bytes())?;
            }
            Bytecode::Return(src) => {
                buf.write_all(&[0xE, *src as u8])?;
            }
            Bytecode::Copy(dst, src) => {
                buf.write_all(&[0xF, *dst as u8, *src as u8])?;
            }
        }
        Ok(())
    }
}

use crate::utils::bytecode::Bytecode;
use crate::utils::stages::{Callable, Elysia};
use std::io::Write;

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
        buf.write_all(&[self.args as u8, self.registers])?;
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
                buf.write_all(&[0x0, *dst, *idx as u8])?;
            }
            Bytecode::Field(dst, src, id) => {
                buf.write_all(&[0x1, *dst, *src])?;
                buf.write_all(&(*id as u32).to_le_bytes())?;
            }
            Bytecode::Unpack(dst, src, idx) => {
                buf.write_all(&[0x2, *dst, *src])?;
                buf.write_all(&(*idx as u32).to_le_bytes())?;
            }
            Bytecode::Pointer(dst, pt, ptr) => {
                buf.write_all(&[0x3, *dst, pt.clone() as u8, *ptr as u8])?;
            }
            Bytecode::Load(dst, idx) => {
                buf.write_all(&[0x4, *dst, *idx as u8])?;
            }
            Bytecode::Binary(dst, lhs, op, rhs) => {
                buf.write_all(&[0x5, *dst, *lhs, op.clone() as u8, *rhs])?;
            }
            Bytecode::Unary(dst, op, src) => {
                buf.write_all(&[0x6, *dst, op.clone() as u8, *src])?;
            }
            Bytecode::Call(dst, src, args) => {
                buf.write_all(&[0x7, *dst, *src, args.len() as u8])?;
                buf.write_all(args)?;
            }
            Bytecode::List(dst, args) => {
                buf.write_all(&[0x8, *dst, args.len() as u8])?;
                buf.write_all(args)?;
            }
            Bytecode::Tuple(dst, args) => {
                buf.write_all(&[0x9, *dst, args.len() as u8])?;
                buf.write_all(args)?;
            }
            Bytecode::Index(dst, src, index) => {
                buf.write_all(&[0xA, *dst, *src, *index])?;
            }
            Bytecode::Method(dst, src, id, args) => {
                buf.write_all(&[0xB, *dst, *src])?;
                buf.write_all(&(*id as u32).to_le_bytes())?;
                buf.write_all(args)?;
            }
            Bytecode::Branch(cond, yes, no) => {
                buf.write_all(&[0xC, *cond])?;
                buf.write_all(&(*yes as u32).to_le_bytes())?;
                buf.write_all(&(*no as u32).to_le_bytes())?;
            }
            Bytecode::Jump(target) => {
                buf.write_all(&[0xD])?;
                buf.write_all(&(*target as u32).to_le_bytes())?;
            }
            Bytecode::Return(src) => {
                buf.write_all(&[0xE, *src])?;
            }
            Bytecode::Copy(dst, src) => {
                buf.write_all(&[0xF, *dst, *src])?;
            }
        }
        Ok(())
    }
}

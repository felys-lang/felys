use crate::utils::bytecode::{Bytecode, Index, Reg};
use crate::utils::group::Group;
use crate::utils::ir::{Const, Pointer};
use crate::utils::stages::{Callable, Elysia};
use crate::{BinOp, UnaOp};
use std::io::Write;

impl Elysia {
    pub fn dump<W: Write>(&self, buf: &mut W) -> std::io::Result<()> {
        self.main.dump(buf)?;

        buf.write_all(&Index::try_from(self.text.len()).unwrap().to_be_bytes())?;
        for callable in self.text.iter() {
            callable.dump(buf)?;
        }

        buf.write_all(&Index::try_from(self.data.len()).unwrap().to_be_bytes())?;
        for constant in self.data.iter() {
            constant.dump(buf)?;
        }

        buf.write_all(&Index::try_from(self.groups.len()).unwrap().to_be_bytes())?;
        for group in self.groups.iter() {
            group.dump(buf)?;
        }

        Ok(())
    }
}

impl Group {
    fn dump<W: Write>(&self, buf: &mut W) -> std::io::Result<()> {
        buf.write_all(&Index::try_from(self.indices.len()).unwrap().to_be_bytes())?;
        for (id, idx) in self.indices.iter() {
            buf.write_all(&id.to_be_bytes())?;
            buf.write_all(&idx.to_be_bytes())?;
        }

        buf.write_all(&Index::try_from(self.methods.len()).unwrap().to_be_bytes())?;
        for (id, idx) in self.methods.iter() {
            buf.write_all(&id.to_be_bytes())?;
            buf.write_all(&idx.to_be_bytes())?;
        }

        Ok(())
    }
}

impl Const {
    fn dump<W: Write>(&self, buf: &mut W) -> std::io::Result<()> {
        match self {
            Const::Int(x) => {
                buf.write_all(&[0x0])?;
                buf.write_all(&x.to_be_bytes())
            }
            Const::Float(x) => {
                buf.write_all(&[0x1])?;
                buf.write_all(&x.to_be_bytes())
            }
            Const::Bool(x) => buf.write_all(&[0x2, *x as u8]),
            Const::Str(x) => {
                buf.write_all(&[0x3])?;
                buf.write_all(&Index::try_from(x.len()).unwrap().to_be_bytes())?;
                buf.write_all(x.as_bytes())
            }
        }
    }
}

impl Callable {
    fn dump<W: Write>(&self, buf: &mut W) -> std::io::Result<()> {
        buf.write_all(&[self.args, self.registers])?;
        buf.write_all(&Index::try_from(self.bytecodes.len()).unwrap().to_be_bytes())?;
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
                buf.write_all(&[0x0, *dst])?;
                buf.write_all(&idx.to_be_bytes())?;
            }
            Bytecode::Field(dst, src, id) => {
                buf.write_all(&[0x1, *dst, *src])?;
                buf.write_all(&id.to_be_bytes())?;
            }
            Bytecode::Unpack(dst, src, idx) => {
                buf.write_all(&[0x2, *dst, *src])?;
                buf.write_all(&idx.to_be_bytes())?;
            }
            Bytecode::Pointer(dst, pt, ptr) => {
                buf.write_all(&[0x3, *dst, pt.into()])?;
                buf.write_all(&ptr.to_be_bytes())?;
            }
            Bytecode::Load(dst, idx) => {
                buf.write_all(&[0x4, *dst])?;
                buf.write_all(&idx.to_be_bytes())?;
            }
            Bytecode::Binary(dst, lhs, op, rhs) => {
                buf.write_all(&[0x5, *dst, *lhs, op.into(), *rhs])?;
            }
            Bytecode::Unary(dst, op, src) => {
                buf.write_all(&[0x6, *dst, op.into(), *src])?;
            }
            Bytecode::Call(dst, src, args) => {
                buf.write_all(&[0x7, *dst, *src, Reg::try_from(args.len()).unwrap()])?;
                buf.write_all(args)?;
            }
            Bytecode::List(dst, args) => {
                buf.write_all(&[0x8, *dst, Reg::try_from(args.len()).unwrap()])?;
                buf.write_all(args)?;
            }
            Bytecode::Tuple(dst, args) => {
                buf.write_all(&[0x9, *dst, Reg::try_from(args.len()).unwrap()])?;
                buf.write_all(args)?;
            }
            Bytecode::Index(dst, src, index) => {
                buf.write_all(&[0xA, *dst, *src, *index])?;
            }
            Bytecode::Method(dst, src, id, args) => {
                buf.write_all(&[0xB, *dst, *src])?;
                buf.write_all(&(*id).to_be_bytes())?;
                buf.write_all(&[Reg::try_from(args.len()).unwrap()])?;
                buf.write_all(args)?;
            }
            Bytecode::Branch(cond, yes, no) => {
                buf.write_all(&[0xC, *cond])?;
                buf.write_all(&(*yes).to_be_bytes())?;
                buf.write_all(&(*no).to_be_bytes())?;
            }
            Bytecode::Jump(target) => {
                buf.write_all(&[0xD])?;
                buf.write_all(&(*target).to_be_bytes())?;
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

impl From<&BinOp> for u8 {
    fn from(value: &BinOp) -> Self {
        match value {
            BinOp::Or => 0x0,
            BinOp::And => 0x1,
            BinOp::Gt => 0x2,
            BinOp::Ge => 0x3,
            BinOp::Lt => 0x4,
            BinOp::Le => 0x5,
            BinOp::Eq => 0x6,
            BinOp::Ne => 0x7,
            BinOp::Add => 0x8,
            BinOp::Sub => 0x9,
            BinOp::Mul => 0xA,
            BinOp::Div => 0xB,
            BinOp::Mod => 0xC,
            BinOp::Dot => 0xD,
        }
    }
}

impl From<&UnaOp> for u8 {
    fn from(value: &UnaOp) -> Self {
        match value {
            UnaOp::Not => 0x0,
            UnaOp::Pos => 0x1,
            UnaOp::Neg => 0x2,
        }
    }
}

impl From<&Pointer> for u8 {
    fn from(value: &Pointer) -> Self {
        match value {
            Pointer::Group => 0x0,
            Pointer::Function => 0x1,
            Pointer::Rust => 0x2,
        }
    }
}

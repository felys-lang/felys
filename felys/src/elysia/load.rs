use crate::utils::ast::{BinOp, UnaOp};
use crate::utils::bytecode::Bytecode;
use crate::utils::function::{Const, Pointer};
use crate::utils::group::Group;
use crate::utils::stages::{Callable, III};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read, Result};

impl III {
    pub fn load<T: Load>(src: &mut T) -> Result<III> {
        Ok(III {
            main: Callable::load(src)?,
            text: {
                let len = src.u32()?;
                let mut text = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    text.push(Callable::load(src)?);
                }
                text
            },
            data: {
                let len = src.u32()?;
                let mut data = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    data.push(Const::load(src)?);
                }
                data
            },
            groups: {
                let len = src.u32()?;
                let mut groups = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    groups.push(Group::load(src)?);
                }
                groups
            },
        })
    }
}

impl Group {
    fn load<T: Load>(src: &mut T) -> Result<Group> {
        let group = Group {
            indices: {
                let len = src.u32()?;
                let mut indices = HashMap::new();
                for _ in 0..len {
                    indices.insert(src.u32()?, src.u32()?);
                }
                indices
            },
            methods: {
                let len = src.u32()?;
                let mut methods = HashMap::new();
                for _ in 0..len {
                    methods.insert(src.u32()?, src.u32()?);
                }
                methods
            },
        };
        Ok(group)
    }
}

impl Const {
    fn load<T: Load>(src: &mut T) -> Result<Const> {
        let tag = src.u8()?;
        let constant = match tag {
            0x0 => Const::Int(src.i32()?),
            0x1 => Const::Float(src.u32()?),
            0x2 => Const::Bool(src.u8()? != 0),
            0x3 => {
                let s = String::from_utf8(src.str()?)
                    .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
                Const::Str(s.into())
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, tag.to_string())),
        };
        Ok(constant)
    }
}

impl Callable {
    fn load<T: Load>(src: &mut T) -> Result<Callable> {
        let callable = Callable {
            args: src.u8()?,
            registers: src.u8()?,
            bytecodes: {
                let len = src.u32()?;
                let mut bytecodes = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    bytecodes.push(Bytecode::load(src)?);
                }
                bytecodes
            },
        };
        Ok(callable)
    }
}

impl Bytecode {
    fn load<T: Load>(src: &mut T) -> Result<Bytecode> {
        let tag = src.u8()?;
        let bytecode = match tag {
            0x0 => Bytecode::Arg(src.u8()?, src.u32()?),
            0x1 => Bytecode::Field(src.u8()?, src.u8()?, src.u32()?),
            0x2 => Bytecode::Unpack(src.u8()?, src.u8()?, src.u32()?),
            0x3 => Bytecode::Pointer(src.u8()?, Pointer::load(src)?, src.u32()?),
            0x4 => Bytecode::Load(src.u8()?, src.u32()?),
            0x5 => Bytecode::Binary(src.u8()?, src.u8()?, BinOp::load(src)?, src.u8()?),
            0x6 => Bytecode::Unary(src.u8()?, UnaOp::load(src)?, src.u8()?),
            0x7 => Bytecode::Call(src.u8()?, src.u8()?, src.vec()?),
            0x8 => Bytecode::List(src.u8()?, src.vec()?),
            0x9 => Bytecode::Tuple(src.u8()?, src.vec()?),
            0xA => Bytecode::Index(src.u8()?, src.u8()?, src.u8()?),
            0xB => Bytecode::Method(src.u8()?, src.u8()?, src.u32()?, src.vec()?),
            0xC => Bytecode::Branch(src.u8()?, src.u32()?, src.u32()?),
            0xD => Bytecode::Jump(src.u32()?),
            0xE => Bytecode::Return(src.u8()?),
            0xF => Bytecode::Copy(src.u8()?, src.u8()?),
            _ => return Err(Error::new(ErrorKind::InvalidData, tag.to_string())),
        };

        Ok(bytecode)
    }
}

impl Pointer {
    fn load<T: Load>(src: &mut T) -> Result<Pointer> {
        let x = src.u8()?;
        let pt = match x {
            0x0 => Pointer::Group,
            0x1 => Pointer::Function,
            0x2 => Pointer::Rust,
            _ => return Err(Error::new(ErrorKind::InvalidData, x.to_string())),
        };
        Ok(pt)
    }
}

impl BinOp {
    fn load<T: Load>(src: &mut T) -> Result<BinOp> {
        let x = src.u8()?;
        let op = match x {
            0x0 => BinOp::Or,
            0x1 => BinOp::And,
            0x2 => BinOp::Gt,
            0x3 => BinOp::Ge,
            0x4 => BinOp::Lt,
            0x5 => BinOp::Le,
            0x6 => BinOp::Eq,
            0x7 => BinOp::Ne,
            0x8 => BinOp::Add,
            0x9 => BinOp::Sub,
            0xA => BinOp::Mul,
            0xB => BinOp::Div,
            0xC => BinOp::Mod,
            0xD => BinOp::At,
            _ => {
                return Err(Error::new(ErrorKind::InvalidData, x.to_string()));
            }
        };
        Ok(op)
    }
}

impl UnaOp {
    fn load<T: Load>(src: &mut T) -> Result<UnaOp> {
        let x = src.u8()?;
        let op = match x {
            0x0 => UnaOp::Not,
            0x1 => UnaOp::Pos,
            0x2 => UnaOp::Neg,
            _ => return Err(Error::new(ErrorKind::InvalidData, x.to_string())),
        };
        Ok(op)
    }
}

pub trait Load: Read {
    fn u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    fn str(&mut self) -> Result<Vec<u8>> {
        let len = self.u32()?;
        let mut vec = vec![0; len as usize];
        self.read_exact(&mut vec)?;
        Ok(vec)
    }

    fn vec(&mut self) -> Result<Vec<u8>> {
        let len = self.u8()?;
        let mut vec = vec![0; len as usize];
        self.read_exact(&mut vec)?;
        Ok(vec)
    }
}

impl<T: Read> Load for T {}

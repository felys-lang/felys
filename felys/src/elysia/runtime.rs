use crate::cyrene::Const;
use crate::demiurge::{Bytecode, Reg};
use crate::elysia::{Callable, Elysia};
use crate::error::Fault;
use std::rc::Rc;

#[derive(Clone)]
enum Object {
    Idx(Idx, usize),
    List(Rc<[Object]>),
    Tuple(Rc<[Object]>),
    Group(usize, Rc<[Object]>),
    Str(Rc<str>),
    Int(isize),
    Float(f64),
    Bool(bool),
    Void,
}

#[derive(Clone, Eq, PartialEq)]
enum Idx {
    Function,
    Group,
}

impl Object {
    fn list(&self) -> Result<&[Object], Fault> {
        if let Object::List(x) = self {
            Ok(x)
        } else {
            Err(Fault::Runtime)
        }
    }

    fn group(&self) -> Result<(usize, &[Object]), Fault> {
        if let Object::Group(x, elements) = self {
            Ok((*x, elements))
        } else {
            Err(Fault::Runtime)
        }
    }

    fn idx(&self) -> Result<(Idx, usize), Fault> {
        if let Object::Idx(ty, idx) = self {
            Ok((ty.clone(), *idx))
        } else {
            Err(Fault::Runtime)
        }
    }

    fn bool(&self) -> Result<bool, Fault> {
        if let Object::Bool(x) = self {
            Ok(*x)
        } else {
            Err(Fault::Runtime)
        }
    }

    fn int(&self) -> Result<isize, Fault> {
        if let Object::Int(x) = self {
            Ok(*x)
        } else {
            Err(Fault::Runtime)
        }
    }
}

struct Frame {
    offset: usize,
    pc: usize,
    registers: Box<[Object]>,
}

impl Frame {
    fn load(&self, reg: usize) -> Result<Object, Fault> {
        self.registers.get(reg).cloned().ok_or(Fault::Runtime)
    }

    fn store(&mut self, reg: usize, obj: Object) -> Result<(), Fault> {
        *self.registers.get_mut(reg).ok_or(Fault::Runtime)? = obj;
        Ok(())
    }
}

struct Runtime {
    rets: Vec<Reg>,
    stack: Vec<Frame>,
}

impl Runtime {
    fn active(&mut self) -> Result<&mut Frame, Fault> {
        self.stack.last_mut().ok_or(Fault::Runtime)
    }

    fn call(
        &mut self,
        dst: Reg,
        idx: usize,
        callable: &Callable,
        args: &[Reg],
    ) -> Result<(), Fault> {
        if args.len() != callable.args {
            return Err(Fault::Runtime);
        }
        let frame = self.active()?;
        let mut registers = vec![Object::Void; callable.registers];
        for arg in args {
            let obj = frame.load(*arg)?;
            registers.push(obj);
        }
        self.rets.push(dst);
        let frame = Frame {
            offset: idx,
            pc: 0,
            registers: registers.into(),
        };
        self.stack.push(frame);
        Ok(())
    }

    fn ret(&mut self, src: Reg) -> Result<(), Fault> {
        let obj = self.active()?.load(src)?;
        self.stack.pop();
        let dst = self.rets.pop().ok_or(Fault::Runtime)?;
        self.active()?.store(dst, obj)
    }
}

impl Bytecode {
    fn exec(&self, elysia: &Elysia, rt: &mut Runtime) -> Result<(), Fault> {
        match self {
            Bytecode::Field(_, _, _) => return Err(Fault::NotImplemented),
            Bytecode::Group(dst, idx) => {
                let frame = rt.active()?;
                let obj = Object::Idx(Idx::Group, *idx);
                frame.store(*dst, obj)?;
            }
            Bytecode::Function(dst, idx) => {
                let frame = rt.active()?;
                let obj = Object::Idx(Idx::Function, *idx);
                frame.store(*dst, obj)?;
            }
            Bytecode::Load(dst, idx) => {
                let frame = rt.active()?;
                let obj = elysia.data.get(*idx).ok_or(Fault::Runtime)?.into();
                frame.store(*dst, obj)?;
            }
            Bytecode::Binary(_, _, _, _) => return Err(Fault::NotImplemented),
            Bytecode::Unary(_, _, _) => return Err(Fault::NotImplemented),
            Bytecode::Call(dst, src, args) => {
                let frame = rt.active()?;
                let (ty, idx) = frame.load(*src)?.idx()?;
                match ty {
                    Idx::Function => {
                        let callable = elysia.text.get(idx).ok_or(Fault::Runtime)?;
                        rt.call(*dst, idx, callable, args)?
                    }
                    Idx::Group => {
                        let group = elysia.lookup.get(idx).ok_or(Fault::Runtime)?;
                        if group.indices.len() != args.len() {
                            return Err(Fault::Runtime);
                        }
                        let mut elements = Vec::with_capacity(args.len());
                        for arg in args {
                            let obj = frame.load(*arg)?;
                            elements.push(obj);
                        }
                        frame.store(*dst, Object::Group(idx, elements.into()))?;
                    }
                };
            }
            Bytecode::List(dst, args) => {
                let frame = rt.active()?;
                let mut elements = Vec::with_capacity(args.len());
                for arg in args {
                    let obj = frame.load(*arg)?;
                    elements.push(obj);
                }
                frame.store(*dst, Object::List(elements.into()))?;
            }
            Bytecode::Tuple(dst, args) => {
                let frame = rt.active()?;
                let mut elements = Vec::with_capacity(args.len());
                for arg in args {
                    let obj = frame.load(*arg)?;
                    elements.push(obj);
                }
                frame.store(*dst, Object::Tuple(elements.into()))?;
            }
            Bytecode::Index(dst, src, index) => {
                let frame = rt.active()?;
                let idx = frame.load(*index)?.int()?;
                if idx < 0 {
                    return Err(Fault::Runtime);
                }
                let obj = frame
                    .load(*src)?
                    .list()?
                    .get(idx as usize)
                    .cloned()
                    .ok_or(Fault::Runtime)?;
                frame.store(*dst, obj)?;
            }
            Bytecode::Method(dst, src, id, args) => {
                let frame = rt.active()?;
                let (idx, _) = frame.load(*src)?.group()?;
                let idx = elysia
                    .lookup
                    .get(idx)
                    .ok_or(Fault::Runtime)?
                    .methods
                    .get(id)
                    .ok_or(Fault::Runtime)?;
                let callable = elysia.text.get(*idx).ok_or(Fault::Runtime)?;
                rt.call(*dst, *idx, callable, args)?;
            }
            Bytecode::Branch(cond, yes, no) => {
                let frame = rt.active()?;
                if frame.load(*cond)?.bool()? {
                    frame.pc = *yes;
                } else {
                    frame.pc = *no;
                }
            }
            Bytecode::Jump(target) => rt.active()?.pc = *target,
            Bytecode::Return(src) => rt.ret(*src)?,
            Bytecode::Copy(dst, src) => {
                let frame = rt.active()?;
                let obj = frame.load(*src)?;
                frame.store(*dst, obj)?;
            }
        }
        Ok(())
    }
}

impl From<&Const> for Object {
    fn from(value: &Const) -> Self {
        match value {
            Const::Int(x) => Object::Int(*x),
            Const::Float(x) => Object::Float(f64::from_bits(*x)),
            Const::Bool(x) => Object::Bool(*x),
            Const::Str(x) => Object::Str(x.clone()),
        }
    }
}

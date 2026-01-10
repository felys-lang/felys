use crate::cyrene::Const;
use crate::demiurge::{Bytecode, Reg};
use crate::elysia::runtime::object::{Object, Pointer};
use crate::elysia::{Callable, Elysia};
use crate::error::Fault;

impl Elysia {
    pub fn exec(&self, args: Object) -> Result<Object, Fault> {
        let mut runtime = self.init(args)?;
        loop {
            let (idx, frame) = runtime.active();
            let callable = match idx {
                Some(x) => self.text.get(x).ok_or(Fault::Runtime)?,
                None => &self.main,
            };
            let bytecode = callable.bytecodes.get(frame.pc).ok_or(Fault::Runtime)?;
            frame.pc += 1;
            if let Some(exit) = bytecode.exec(self, &mut runtime)? {
                break Ok(exit);
            }
        }
    }

    fn init(&self, args: Object) -> Result<Runtime, Fault> {
        let runtime = Runtime {
            rets: vec![],
            main: self.main.frame(vec![args])?,
            stack: vec![],
        };
        Ok(runtime)
    }
}

struct Runtime {
    rets: Vec<Reg>,
    main: Frame,
    stack: Vec<(usize, Frame)>,
}

impl Runtime {
    fn active(&mut self) -> (Option<usize>, &mut Frame) {
        self.stack
            .last_mut()
            .map(|(idx, frame)| (Some(*idx), frame))
            .unwrap_or((None, &mut self.main))
    }

    fn call(
        &mut self,
        dst: Reg,
        idx: usize,
        callable: &Callable,
        args: Vec<Object>,
    ) -> Result<(), Fault> {
        let new = callable.frame(args)?;
        self.rets.push(dst);
        self.stack.push((idx, new));
        Ok(())
    }

    fn ret(&mut self, src: Reg) -> Result<Option<Object>, Fault> {
        let obj = self.active().1.load(src)?;
        if self.stack.pop().is_none() {
            return Ok(Some(obj));
        };
        let dst = self.rets.pop().ok_or(Fault::Runtime)?;
        self.active().1.store(dst, obj)?;
        Ok(None)
    }
}

struct Frame {
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

impl Callable {
    fn frame(&self, mut args: Vec<Object>) -> Result<Frame, Fault> {
        if args.len() != self.args {
            return Err(Fault::Runtime);
        }
        args.resize(self.registers, Object::Void);
        let frame = Frame {
            pc: 0,
            registers: args.into(),
        };
        Ok(frame)
    }
}

impl Bytecode {
    fn exec(&self, elysia: &Elysia, rt: &mut Runtime) -> Result<Option<Object>, Fault> {
        match self {
            Bytecode::Field(_, _, _) => return Err(Fault::NotImplemented),
            Bytecode::Group(dst, idx) => {
                let (_, frame) = rt.active();
                let obj = Object::Pointer(Pointer::Group, *idx);
                frame.store(*dst, obj)?;
            }
            Bytecode::Function(dst, idx) => {
                let (_, frame) = rt.active();
                let obj = Object::Pointer(Pointer::Function, *idx);
                frame.store(*dst, obj)?;
            }
            Bytecode::Load(dst, idx) => {
                let (_, frame) = rt.active();
                let obj = elysia.data.get(*idx).ok_or(Fault::Runtime)?.into();
                frame.store(*dst, obj)?;
            }
            Bytecode::Binary(_, _, _, _) => return Err(Fault::NotImplemented),
            Bytecode::Unary(_, _, _) => return Err(Fault::NotImplemented),
            Bytecode::Call(dst, src, args) => {
                let (_, frame) = rt.active();
                let (ty, idx) = frame.load(*src)?.pointer()?;
                match ty {
                    Pointer::Function => {
                        let callable = elysia.text.get(idx).ok_or(Fault::Runtime)?;
                        let mut objs = Vec::with_capacity(callable.registers);
                        for arg in args {
                            let obj = frame.load(*arg)?;
                            objs.push(obj);
                        }
                        rt.call(*dst, idx, callable, objs)?
                    }
                    Pointer::Group => {
                        let group = elysia.router.get(idx).ok_or(Fault::Runtime)?;
                        if group.indices.len() != args.len() {
                            return Err(Fault::Runtime);
                        }
                        let mut objs = Vec::with_capacity(args.len());
                        for arg in args {
                            let obj = frame.load(*arg)?;
                            objs.push(obj);
                        }
                        frame.store(*dst, Object::Group(idx, objs.into()))?;
                    }
                };
            }
            Bytecode::List(dst, args) => {
                let (_, frame) = rt.active();
                let mut objs = Vec::with_capacity(args.len());
                for arg in args {
                    let obj = frame.load(*arg)?;
                    objs.push(obj);
                }
                frame.store(*dst, Object::List(objs.into()))?;
            }
            Bytecode::Tuple(dst, args) => {
                let (_, frame) = rt.active();
                let mut objs = Vec::with_capacity(args.len());
                for arg in args {
                    let obj = frame.load(*arg)?;
                    objs.push(obj);
                }
                frame.store(*dst, Object::Tuple(objs.into()))?;
            }
            Bytecode::Index(dst, src, index) => {
                let (_, frame) = rt.active();
                let list = frame.load(*src)?.list()?;
                let idx = frame.load(*index)?.int()?;
                let idx = if idx >= 0 {
                    idx as usize
                } else {
                    list.len()
                        .checked_sub(idx.unsigned_abs())
                        .ok_or(Fault::Runtime)?
                };
                let obj = list.get(idx).cloned().ok_or(Fault::Runtime)?;
                frame.store(*dst, obj)?;
            }
            Bytecode::Method(dst, src, id, args) => {
                let (_, frame) = rt.active();
                let obj = frame.load(*src)?;
                let (idx, _) = obj.group()?;
                let idx = elysia
                    .router
                    .get(idx)
                    .ok_or(Fault::Runtime)?
                    .methods
                    .get(id)
                    .ok_or(Fault::Runtime)?;
                let callable = elysia.text.get(*idx).ok_or(Fault::Runtime)?;
                let mut objs = Vec::with_capacity(callable.registers);
                objs.push(obj);
                for arg in args {
                    let obj = frame.load(*arg)?;
                    objs.push(obj);
                }
                rt.call(*dst, *idx, callable, objs)?;
            }
            Bytecode::Branch(cond, yes, no) => {
                let (_, frame) = rt.active();
                if frame.load(*cond)?.bool()? {
                    frame.pc = *yes;
                } else {
                    frame.pc = *no;
                }
            }
            Bytecode::Jump(target) => rt.active().1.pc = *target,
            Bytecode::Return(src) => return rt.ret(*src),
            Bytecode::Copy(dst, src) => {
                let (_, frame) = rt.active();
                let obj = frame.load(*src)?;
                frame.store(*dst, obj)?;
            }
        }
        Ok(None)
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

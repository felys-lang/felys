use crate::elysia::fault::Fault;
use crate::elysia::runtime::object::Object;
use crate::utils::bytecode::{Bytecode, Index, Reg};
use crate::utils::ir::{Const, Pointer};
use crate::utils::stages::{Callable, Elysia};

impl Elysia {
    pub fn exec(&self, args: Object, stdout: &mut String) -> Result<String, String> {
        let exit = self
            .kernal(args, stdout)
            .map_err(|e| e.recover(&self.groups))?;
        let mut buf = String::new();
        exit.recover(&mut buf, 0, &self.groups).unwrap();
        Ok(buf)
    }

    fn kernal(&self, args: Object, stdout: &mut String) -> Result<Object, Fault> {
        let mut runtime = self.init(args)?;
        loop {
            let (idx, frame) = runtime.active();
            let bytecode = self.loc(idx).loc(frame.pc);
            frame.pc += 1;
            if let Some(exit) = bytecode.exec(self, &mut runtime, stdout)? {
                break Ok(exit);
            }
        }
    }

    fn init(&self, args: Object) -> Result<Runtime, Fault> {
        let runtime = Runtime {
            args,
            rets: vec![],
            main: self.main.frame(vec![1])?,
            stack: vec![],
        };
        Ok(runtime)
    }

    fn loc(&self, idx: Option<Index>) -> &Callable {
        match idx {
            Some(x) => self.text.get(x as usize).unwrap(),
            None => &self.main,
        }
    }
}

struct Runtime {
    args: Object,
    rets: Vec<Reg>,
    main: Frame,
    stack: Vec<(Index, Frame)>,
}

impl Runtime {
    fn active(&mut self) -> (Option<Index>, &mut Frame) {
        self.stack
            .last_mut()
            .map(|(idx, frame)| (Some(*idx), frame))
            .unwrap_or((None, &mut self.main))
    }

    fn frame(&mut self) -> &mut Frame {
        self.stack
            .last_mut()
            .map(|(_, frame)| frame)
            .unwrap_or(&mut self.main)
    }

    fn arg(&mut self, idx: Index) -> Object {
        if let Some(tmp) = self.stack.pop() {
            let reg = *tmp.1.args.get(idx as usize).unwrap();
            let obj = self.frame().load(reg);
            self.stack.push(tmp);
            obj
        } else {
            self.args.clone()
        }
    }

    fn call(&mut self, dst: Reg, idx: Index, frame: Frame) -> Result<(), Fault> {
        self.rets.push(dst);
        self.stack.push((idx, frame));
        Ok(())
    }

    fn ret(&mut self, src: Reg) -> Result<Option<Object>, Fault> {
        let obj = self.frame().load(src);
        if self.stack.pop().is_none() {
            return Ok(Some(obj));
        };
        let dst = self.rets.pop().unwrap();
        self.frame().store(dst, obj);
        Ok(None)
    }
}

struct Frame {
    pc: Index,
    registers: Box<[Object]>,
    args: Box<[Reg]>,
}

impl Frame {
    fn load(&self, reg: Reg) -> Object {
        self.registers.get(reg as usize).cloned().unwrap()
    }

    fn store(&mut self, reg: Reg, obj: Object) {
        *self.registers.get_mut(reg as usize).unwrap() = obj;
    }

    fn gather(&self, args: &[Reg]) -> Vec<Object> {
        let mut objs = Vec::with_capacity(args.len());
        for arg in args {
            let obj = self.load(*arg);
            objs.push(obj);
        }
        objs
    }
}

impl Callable {
    fn loc(&self, idx: Index) -> &Bytecode {
        self.bytecodes.get(idx as usize).unwrap()
    }

    fn frame(&self, args: Vec<Reg>) -> Result<Frame, Fault> {
        if self.args != args.len() {
            return Err(Fault::NumArgsNotMatch(self.args, args.len()));
        }
        let frame = Frame {
            pc: 0,
            registers: vec![Object::Void; self.registers as usize].into_boxed_slice(),
            args: args.into_boxed_slice(),
        };
        Ok(frame)
    }
}

impl Bytecode {
    fn exec(
        &self,
        elysia: &Elysia,
        rt: &mut Runtime,
        cs: &mut String,
    ) -> Result<Option<Object>, Fault> {
        match self {
            Bytecode::Arg(dst, idx) => {
                let obj = rt.arg(*idx);
                rt.frame().store(*dst, obj);
            }
            Bytecode::Field(dst, src, id) => {
                let frame = rt.frame();
                let tmp = frame.load(*src);
                let (gp, group) = tmp.group()?;
                let idx = elysia
                    .groups
                    .get(gp as usize)
                    .unwrap()
                    .indices
                    .get(id)
                    .unwrap();
                let obj = group.get(*idx).cloned().unwrap();
                frame.store(*dst, obj);
            }
            Bytecode::Unpack(dst, src, idx) => {
                let frame = rt.frame();
                let tmp = frame.load(*src);
                let objs = tmp.tuple()?;
                let obj = objs
                    .get(*idx as usize)
                    .cloned()
                    .ok_or(Fault::NotEnoughToUnpack(tmp, *idx))?;
                frame.store(*dst, obj);
            }
            Bytecode::Pointer(dst, pt, idx) => match pt {
                Pointer::Function => {
                    let obj = Object::Pointer(Pointer::Function, *idx);
                    rt.frame().store(*dst, obj);
                }
                Pointer::Group => {
                    let obj = Object::Pointer(Pointer::Group, *idx);
                    rt.frame().store(*dst, obj);
                }
                Pointer::Rust => {
                    let obj = Object::Pointer(Pointer::Rust, *idx);
                    rt.frame().store(*dst, obj);
                }
            },
            Bytecode::Load(dst, idx) => {
                let obj = elysia.data.get(*idx as usize).unwrap().into();
                rt.frame().store(*dst, obj);
            }
            Bytecode::Binary(dst, lhs, op, rhs) => {
                let frame = rt.frame();
                let l = frame.load(*lhs);
                let r = frame.load(*rhs);
                let obj = l.binary(op, r)?;
                frame.store(*dst, obj);
            }
            Bytecode::Unary(dst, op, src) => {
                let frame = rt.frame();
                let s = frame.load(*src);
                let obj = s.unary(op)?;
                frame.store(*dst, obj);
            }
            Bytecode::Call(dst, src, args) => {
                let frame = rt.frame();
                let (ty, idx) = frame.load(*src).pointer()?;
                match ty {
                    Pointer::Function => {
                        let new = elysia.text.get(idx as usize).unwrap().frame(args.clone())?;
                        rt.call(*dst, idx, new)?
                    }
                    Pointer::Group => {
                        let group = elysia.groups.get(idx as usize).unwrap();
                        let objs = frame.gather(args);
                        let expected = group.indices.len();
                        if expected != args.len() {
                            return Err(Fault::NumArgsNotMatch(expected, args.len()));
                        }
                        frame.store(*dst, Object::Group(idx, objs.into()));
                    }
                    Pointer::Rust => {
                        let f = elysia.rust.get(idx as usize).unwrap();
                        let objs = frame.gather(args);
                        frame.store(*dst, f(objs, elysia, cs));
                    }
                };
            }
            Bytecode::List(dst, args) => {
                let frame = rt.frame();
                let objs = frame.gather(args);
                frame.store(*dst, Object::List(objs.into()));
            }
            Bytecode::Tuple(dst, args) => {
                let frame = rt.frame();
                let objs = frame.gather(args);
                frame.store(*dst, Object::Tuple(objs.into()));
            }
            Bytecode::Index(dst, src, index) => {
                let frame = rt.frame();
                let tmp = frame.load(*src);
                let list = tmp.list()?;
                let int = frame.load(*index).int()?;
                let idx = if int >= 0 {
                    int as usize
                } else {
                    list.len()
                        .checked_sub(int.unsigned_abs())
                        .ok_or(Fault::IndexOutOfBounds(tmp.clone(), int))?
                };
                let obj = list
                    .get(idx)
                    .cloned()
                    .ok_or(Fault::IndexOutOfBounds(tmp, int))?;
                frame.store(*dst, obj);
            }
            Bytecode::Method(dst, src, id, args) => {
                let (gp, _) = rt.frame().load(*src).group()?;
                let idx = elysia
                    .groups
                    .get(gp as usize)
                    .unwrap()
                    .methods
                    .get(id)
                    .unwrap();
                let mut args = args.clone();
                args.push(*src);
                let new = elysia.text.get(*idx).unwrap().frame(args)?;
                rt.call(*dst, *idx as Index, new)?;
            }
            Bytecode::Branch(cond, yes, no) => {
                let frame = rt.frame();
                if frame.load(*cond).bool()? {
                    frame.pc = *yes;
                } else {
                    frame.pc = *no;
                }
            }
            Bytecode::Jump(target) => rt.frame().pc = *target,
            Bytecode::Return(src) => return rt.ret(*src),
            Bytecode::Copy(dst, src) => {
                let frame = rt.frame();
                let obj = frame.load(*src);
                frame.store(*dst, obj);
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

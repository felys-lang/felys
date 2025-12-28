use crate::ast::{Impl, Item, Root};
use crate::cyrene::{Context, Function, Group, Meta, Namespace};
use crate::demiurge::Demiurge;
use crate::error::Fault;
use crate::philia093::Intern;
use std::collections::HashMap;

pub struct Cyrene {
    pub root: Root,
    pub intern: Intern,
}

impl Cyrene {
    pub fn cfg(self) -> Result<Demiurge, Fault> {
        let mut meta = Meta {
            ns: Namespace::new(),
            constructor: Namespace::new(),
            intern: self.intern,
            groups: HashMap::new(),
        };

        for item in self.root.0.iter() {
            item.allocate(&mut meta)?;
        }

        for item in self.root.0.iter() {
            item.attach(&mut meta)?;
        }

        let mut fns = HashMap::new();
        let mut main = None;
        for item in self.root.0.iter() {
            item.cfg(&mut meta, &mut fns, &mut main)?;
        }

        Ok(Demiurge {
            groups: meta.groups,
            fns,
            main: main.ok_or(Fault::EntryNotFound)?,
            intern: meta.intern,
        })
    }
}

impl Item {
    fn allocate(&self, meta: &mut Meta) -> Result<(), Fault> {
        if let Item::Group(id, fields) = self {
            let group = Group::new(fields.iter());
            let gid = meta.constructor.add([*id].iter())?;
            meta.groups.insert(gid, group);
        }
        Ok(())
    }

    fn attach(&self, meta: &mut Meta) -> Result<(), Fault> {
        match self {
            Item::Impl(id, impls) => {
                for implementation in impls.iter() {
                    implementation.attach(*id, meta)?;
                }
            }
            Item::Fn(id, _, _) => {
                meta.ns.add([*id].iter())?;
            }
            _ => {}
        }
        Ok(())
    }

    fn cfg(
        &self,
        meta: &mut Meta,
        fns: &mut HashMap<usize, Function>,
        main: &mut Option<Function>,
    ) -> Result<(), Fault> {
        match self {
            Item::Fn(id, args, block) => {
                let mut f = Function::new();
                let mut ctx = match args {
                    Some(vec) => Context::new(vec.iter()),
                    None => Context::new([].iter()),
                };
                block.ir(&mut f, &mut ctx, meta, None)?;
                let src = meta.ns.get([*id].iter())?;
                fns.insert(src, f);
            }
            Item::Main(args, block) => {
                let mut f = Function::new();
                let mut ctx = Context::new([*args].iter());
                block.ir(&mut f, &mut ctx, meta, None)?;
                *main = Some(f);
            }
            Item::Impl(id, impls) => {
                for implementation in impls.iter() {
                    implementation.cfg(*id, meta, fns)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Impl {
    fn attach(&self, id: usize, meta: &mut Meta) -> Result<(), Fault> {
        match self {
            Impl::Associated(sid, _, _) => {
                meta.ns.add([id, *sid].iter())?;
            }
            Impl::Method(sid, _, _) => {
                let gid = meta.constructor.get([id].iter())?;
                let src = meta.ns.add([id, *sid].iter())?;
                let group = meta.groups.get_mut(&gid).unwrap();
                group.methods.insert(*sid, src);
            }
        }
        Ok(())
    }

    fn cfg(
        &self,
        id: usize,
        meta: &mut Meta,
        fns: &mut HashMap<usize, Function>,
    ) -> Result<(), Fault> {
        let mut f = Function::new();
        match self {
            Impl::Associated(sid, args, block) => {
                let mut ctx = match args {
                    Some(vec) => Context::new(vec.iter()),
                    None => Context::new([].iter()),
                };
                block.ir(&mut f, &mut ctx, meta, None)?;
                let src = meta.ns.get([id, *sid].iter())?;
                fns.insert(src, f);
            }
            Impl::Method(sid, args, block) => {
                let s = meta.intern.id("self");
                let mut ctx = Context::new([s].iter().chain(args));
                block.ir(&mut f, &mut ctx, meta, None)?;
                let src = meta.ns.get([id, *sid].iter())?;
                fns.insert(src, f);
            }
        }
        Ok(())
    }
}

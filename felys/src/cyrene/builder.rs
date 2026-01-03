use crate::ast::{Impl, Item, Root};
use crate::cyrene::meta::{Group, Meta, Namespace};
use crate::demiurge::{Demiurge, Function};
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
            constructors: Namespace::new(),
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
            main: main.ok_or(Fault::MainNotFound)?,
            intern: meta.intern,
        })
    }
}

impl Item {
    fn allocate(&self, meta: &mut Meta) -> Result<(), Fault> {
        if let Item::Group(id, fields) = self {
            let group = Group::new(fields.iter());
            let gid = meta.constructors.add([*id].iter())?;
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
                let function = match args {
                    Some(vec) => block.build(vec.iter(), meta)?,
                    None => block.build([].iter(), meta)?,
                };
                let src = meta.ns.get([*id].iter())?;
                fns.insert(src, function);
            }
            Item::Main(args, block) => *main = Some(block.build([*args].iter(), meta)?),
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
                let gid = meta.constructors.get([id].iter())?;
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
        match self {
            Impl::Associated(sid, args, block) => {
                let function = match args {
                    Some(vec) => block.build(vec.iter(), meta)?,
                    None => block.build([].iter(), meta)?,
                };
                let src = meta.ns.get([id, *sid].iter())?;
                fns.insert(src, function);
            }
            Impl::Method(sid, args, block) => {
                let s = meta.intern.id("self");
                let function = block.build([s].iter().chain(args), meta)?;
                let src = meta.ns.get([id, *sid].iter())?;
                fns.insert(src, function);
            }
        }
        Ok(())
    }
}

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
            if let Item::Group(id, fields) = item {
                let group = Group::new(fields.iter());
                let gid = meta.constructor.add([*id].iter())?;
                meta.groups.insert(gid, group);
            }
        }

        for item in self.root.0.iter() {
            match item {
                Item::Group(_, _) => {}
                Item::Impl(id, impls) => {
                    for implementation in impls.iter() {
                        match implementation {
                            Impl::Associated(sid, _, _) => {
                                meta.ns.add([*id, *sid].iter())?;
                            }
                            Impl::Method(sid, _, _) => {
                                let gid = meta.constructor.get([*id].iter())?;
                                let src = meta.ns.add([*id, *sid].iter())?;
                                let group = meta.groups.get_mut(&gid).unwrap();
                                group.methods.insert(*sid, src);
                            }
                        };
                    }
                }
                Item::Fn(id, _, _) => {
                    meta.ns.add([*id].iter())?;
                }
                Item::Main(_, _) => {}
            }
        }

        let mut fns = HashMap::new();
        let mut entry = None;
        for item in self.root.0.iter() {
            match item {
                Item::Fn(id, args, block) => {
                    let mut f = Function::new();
                    let mut ctx = match args {
                        Some(vec) => Context::new(vec.iter()),
                        None => Context::new([].iter()),
                    };
                    block.ir(&mut f, &mut ctx, &meta, None)?;
                    let src = meta.ns.get([*id].iter())?;
                    fns.insert(src, f);
                }
                Item::Main(args, block) => {
                    let mut f = Function::new();
                    let mut ctx = Context::new([*args].iter());
                    block.ir(&mut f, &mut ctx, &meta, None)?;
                    entry = Some(f);
                }
                Item::Group(_, _) => {}
                Item::Impl(id, impls) => {
                    for implementation in impls.iter() {
                        let mut f = Function::new();
                        match implementation {
                            Impl::Associated(sid, args, block) => {
                                let mut ctx = match args {
                                    Some(vec) => Context::new(vec.iter()),
                                    None => Context::new([].iter()),
                                };
                                block.ir(&mut f, &mut ctx, &meta, None)?;
                                let src = meta.ns.get([*id, *sid].iter())?;
                                fns.insert(src, f);
                            }
                            Impl::Method(sid, args, block) => {
                                let s = meta.intern.id("self");
                                let mut ctx = Context::new([s].iter().chain(args));
                                block.ir(&mut f, &mut ctx, &meta, None)?;
                                let src = meta.ns.get([*id, *sid].iter())?;
                                fns.insert(src, f);
                            }
                        }
                    }
                }
            }
        }

        Ok(Demiurge {
            groups: meta.groups,
            fns,
            main: entry.ok_or(Fault::EntryNotFound)?,
            intern: meta.intern,
        })
    }
}

use crate::cyrene::fault::Fault;
use crate::cyrene::meta::Meta;
use crate::demiurge::Demiurge;
use crate::philia093::Intern;
use crate::utils::ast::{BufVec, Impl, Item, Root};
use crate::utils::function::Function;
use crate::utils::group::Group;
use std::collections::HashMap;

pub struct Cyrene {
    pub root: Root,
    pub intern: Intern,
}

impl Cyrene {
    pub fn cfg(self) -> Result<Demiurge, String> {
        let mut meta = Meta::new(self.intern);

        for item in self.root.0.iter() {
            item.allocate(&mut meta)
                .map_err(|e| e.recover(&meta.intern))?;
        }

        for item in self.root.0.iter() {
            item.attach(&mut meta)
                .map_err(|e| e.recover(&meta.intern))?;
        }

        let mut fns = HashMap::new();
        let mut main = None;
        for item in self.root.0.iter() {
            item.cfg(&mut meta, &mut fns, &mut main)
                .map_err(|e| e.recover(&meta.intern))?;
        }

        Ok(Demiurge {
            gps: meta.groups,
            fns,
            main: main.ok_or(Fault::MainNotFound(self.root).recover(&meta.intern))?,
        })
    }
}

impl Item {
    fn allocate(&self, meta: &mut Meta) -> Result<(), Fault> {
        if let Item::Group(id, fields) = self {
            let group = Group::new(fields.iter().copied().collect());
            let gid = meta
                .namespace
                .allocate(&[], *id)
                .ok_or(Fault::DuplicatePath(BufVec::new([*id], vec![])))?;
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
                meta.namespace
                    .attach(&[], *id)
                    .ok_or(Fault::DuplicatePath(BufVec::new([*id], vec![])))?;
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
                let args = args.as_ref().map(|x| x.vec()).unwrap_or_default();
                let function = block.function(args, meta)?;
                let (_, src) = meta.namespace.get([*id].iter()).unwrap();
                fns.insert(src, function);
            }
            Item::Main(args, block) => *main = Some(block.function(vec![*args], meta)?),
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
                meta.namespace
                    .attach(&[id], *sid)
                    .ok_or(Fault::DuplicatePath(BufVec::new([id], vec![*sid])))?;
            }
            Impl::Method(sid, _, _) => {
                let (_, gid) = meta.namespace.get([id].iter()).unwrap();
                let src = meta
                    .namespace
                    .attach(&[id], *sid)
                    .ok_or(Fault::DuplicatePath(BufVec::new([id], vec![*sid])))?;
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
                let args = args.as_ref().map(|x| x.vec()).unwrap_or_default();
                let function = block.function(args, meta)?;
                let (_, src) = meta.namespace.get([id, *sid].iter()).unwrap();
                fns.insert(src, function);
            }
            Impl::Method(sid, args, block) => {
                let s = meta.intern.id("self");
                let args = [s].iter().chain(args).cloned().collect();
                let function = block.function(args, meta)?;
                let (_, src) = meta.namespace.get([id, *sid].iter()).unwrap();
                fns.insert(src, function);
            }
        }
        Ok(())
    }
}

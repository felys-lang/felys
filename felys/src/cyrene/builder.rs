use crate::cyrene::fault::Fault;
use crate::cyrene::meta::Meta;
use crate::demiurge::Demiurge;
use crate::philia093::Intern;
use crate::utils::ast::{BufVec, Impl, Item, Root};
use crate::utils::group::Group;

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

        for item in self.root.0.iter() {
            item.cfg(&mut meta).map_err(|e| e.recover(&meta.intern))?;
        }

        Ok(Demiurge {
            gps: meta.groups,
            fns: meta.functions,
            main: meta
                .main
                .ok_or(Fault::MainNotFound(self.root).recover(&meta.intern))?,
        })
    }
}

impl Item {
    fn allocate(&self, meta: &mut Meta) -> Result<(), Fault> {
        if let Item::Group(id, fields) = self {
            let gp = meta
                .namespace
                .allocate(&[], *id)
                .ok_or(Fault::DuplicatePath(BufVec::new([*id], vec![])))?;
            let group = Group::new(fields.iter().copied().collect());
            meta.groups.insert(gp, group);
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

    fn cfg(&self, meta: &mut Meta) -> Result<(), Fault> {
        match self {
            Item::Fn(id, args, block) => {
                let args = args.as_ref().map(|x| x.vec()).unwrap_or_default();
                let function = block.function(args, meta)?;
                let (_, ptr) = meta.namespace.get([*id].iter()).unwrap();
                meta.functions.insert(ptr, function);
            }
            Item::Main(args, block) => meta.main = Some(block.function(vec![*args], meta)?),
            Item::Impl(id, impls) => {
                for implementation in impls.iter() {
                    implementation.cfg(*id, meta)?;
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
            Impl::Associated(secondary, _, _) => {
                meta.namespace
                    .attach(&[id], *secondary)
                    .ok_or(Fault::DuplicatePath(BufVec::new([id], vec![*secondary])))?;
            }
            Impl::Method(secondary, _, _) => {
                let ptr = meta
                    .namespace
                    .attach(&[id], *secondary)
                    .ok_or(Fault::DuplicatePath(BufVec::new([id], vec![*secondary])))?;
                let (_, gp) = meta.namespace.get([id].iter()).unwrap();
                let group = meta.groups.get_mut(&gp).unwrap();
                group.methods.insert(*secondary, ptr);
            }
        }
        Ok(())
    }

    fn cfg(&self, id: usize, meta: &mut Meta) -> Result<(), Fault> {
        match self {
            Impl::Associated(secondary, args, block) => {
                let args = args.as_ref().map(|x| x.vec()).unwrap_or_default();
                let function = block.function(args, meta)?;
                let (_, ptr) = meta.namespace.get([id, *secondary].iter()).unwrap();
                meta.functions.insert(ptr, function);
            }
            Impl::Method(secondary, args, block) => {
                let mut args = args.clone();
                args.push(meta.intern.id("self"));
                let function = block.function(args, meta)?;
                let (_, ptr) = meta.namespace.get([id, *secondary].iter()).unwrap();
                meta.functions.insert(ptr, function);
            }
        }
        Ok(())
    }
}

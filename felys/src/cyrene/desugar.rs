use crate::cyrene::error::Error;
use crate::philia093::Intern;
use crate::utils::ast::{Block, Impl, Item};
use crate::utils::group::Group;
use crate::utils::namespace::Namespace;
use crate::utils::stages::{I, II};
use std::collections::HashMap;

impl I {
    pub fn desugar(self) -> Result<II, String> {
        let mut intern = self.intern;
        let mut namespace = Namespace::init(&mut intern);
        let mut functions = HashMap::new();
        let mut groups = HashMap::new();

        for item in self.root.0.iter() {
            item.allocate(&mut namespace, &mut groups)?;
        }

        let mut main = Err(Error::MainNotFound);
        for item in self.root.0.into_iter() {
            item.attach(&mut intern, &mut namespace, &mut functions, &mut main)?;
        }

        Ok(II {
            namespace,
            groups,
            functions,
            main: main.map_err(|e| e.recover(&intern))?,
            intern,
        })
    }
}

impl Item {
    fn allocate(
        &self,
        namespace: &mut Namespace,
        groups: &mut HashMap<usize, Group>,
    ) -> Result<(), &'static str> {
        if let Item::Group(id, fields) = self {
            let gp = namespace.allocate(&[], *id).ok_or("duplicated path")?;
            let group = Group::new(fields.iter());
            groups.insert(gp, group);
        }
        Ok(())
    }

    fn attach(
        self,
        intern: &mut Intern,
        namespace: &mut Namespace,
        functions: &mut HashMap<usize, (Vec<usize>, Block)>,
        main: &mut Result<(usize, Block), Error>,
    ) -> Result<(), &'static str> {
        match self {
            Item::Impl(id, impls) => {
                for implementation in impls.into_iter() {
                    implementation.attach(id, intern, namespace, functions)?;
                }
            }
            Item::Fn(id, args, block) => {
                let ptr = namespace.attach(&[], id).ok_or("duplicated path")?;
                let args = args.map(|x| x.vec()).unwrap_or_default();
                functions.insert(ptr, (args, block));
            }
            Item::Main(args, block) => *main = Ok((args, block)),
            _ => {}
        }
        Ok(())
    }
}

impl Impl {
    fn attach(
        self,
        id: usize,
        intern: &mut Intern,
        namespace: &mut Namespace,
        functions: &mut HashMap<usize, (Vec<usize>, Block)>,
    ) -> Result<(), &'static str> {
        match self {
            Impl::Associated(secondary, args, block) => {
                let ptr = namespace
                    .attach(&[id], secondary)
                    .ok_or("duplicated path")?;
                let args = args.map(|x| x.vec()).unwrap_or_default();
                functions.insert(ptr, (args, block));
            }
            Impl::Method(secondary, mut args, block) => {
                let ptr = namespace
                    .attach(&[id], secondary)
                    .ok_or("duplicated path")?;
                args.push(intern.id("self"));
                functions.insert(ptr, (args, block));
            }
        }
        Ok(())
    }
}

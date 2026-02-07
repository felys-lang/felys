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
            item.allocate(&mut namespace, &mut groups)
                .map_err(|e| e.recover(&intern))?;
        }

        let mut main = Err(Error::MainNotFound);
        for item in self.root.0.into_iter() {
            item.attach(
                &mut intern,
                &mut namespace,
                &mut functions,
                &mut groups,
                &mut main,
            )
            .map_err(|e| e.recover(&intern))?;
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
    ) -> Result<(), Error> {
        if let Item::Group(id, fields) = self {
            let gp = namespace
                .allocate(&[], *id)
                .ok_or(Error::RedeclaredItem(self.clone()))?;
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
        groups: &mut HashMap<usize, Group>,
        main: &mut Result<(usize, Block), Error>,
    ) -> Result<(), Error> {
        let ptr = self.ptr(namespace, main)?;
        match self {
            Item::Group(_, _) => {}
            Item::Impl(id, impls) => {
                for implementation in impls.into_iter() {
                    implementation.attach(id, intern, namespace, functions, groups)?;
                }
            }
            Item::Fn(_, args, block) => {
                let args = args.map(|x| x.vec()).unwrap_or_default();
                functions.insert(ptr.unwrap(), (args, block));
            }
            Item::Main(args, block) => *main = Ok((args, block)),
        }
        Ok(())
    }

    fn ptr(
        &self,
        namespace: &mut Namespace,
        main: &mut Result<(usize, Block), Error>,
    ) -> Result<Option<usize>, Error> {
        match self {
            Item::Group(_, _) | Item::Impl(_, _) => Ok(None),
            Item::Fn(id, _, _) => namespace
                .attach(&[], *id)
                .ok_or(Error::RedeclaredItem(self.clone()))
                .map(Some),
            Item::Main(_, _) => {
                if main.is_ok() {
                    Err(Error::RedeclaredItem(self.clone()))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

impl Impl {
    fn attach(
        self,
        id: usize,
        intern: &mut Intern,
        namespace: &mut Namespace,
        functions: &mut HashMap<usize, (Vec<usize>, Block)>,
        groups: &mut HashMap<usize, Group>,
    ) -> Result<(), Error> {
        let ptr = self.ptr(id, namespace)?;
        match self {
            Impl::Associated(_, args, block) => {
                let args = args.map(|x| x.vec()).unwrap_or_default();
                functions.insert(ptr, (args, block));
            }
            Impl::Method(secondary, mut args, block) => {
                args.push(intern.id("self"));
                functions.insert(ptr, (args, block));
                let (_, gp) = namespace.get([id].iter()).unwrap();
                groups.get_mut(&gp).unwrap().attach(secondary, ptr);
            }
        }
        Ok(())
    }

    fn ptr(&self, id: usize, namespace: &mut Namespace) -> Result<usize, Error> {
        match self {
            Impl::Associated(x, _, _) | Impl::Method(x, _, _) => namespace
                .attach(&[id], *x)
                .ok_or(Error::RedeclaredImpl(self.clone())),
        }
    }
}

use crate::ast::{BufVec, Item, Root};
use crate::cyrene::{Context, Function, Namespace};
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
        let mut ns = Namespace::new();
        for item in self.root.0.iter() {
            if let Item::Fn(id, _, _) = item {
                let path = BufVec::new([*id], Vec::new());
                ns.add(&path)?
            }
        }

        let mut functions = HashMap::new();
        let mut entry = None;
        for item in self.root.0.iter() {
            match item {
                Item::Fn(id, args, block) => {
                    let mut f = Function::new();
                    let mut ctx = match args {
                        Some(vec) => Context::new(vec.iter()),
                        None => Context::new([].iter()),
                    };
                    block.ir(&mut f, &mut ctx, &ns)?;
                    functions.insert(*id, f);
                }
                Item::Main(args, block) => {
                    let mut f = Function::new();
                    let args = [*args];
                    let mut ctx = Context::new(args.iter());
                    block.ir(&mut f, &mut ctx, &ns)?;
                    entry = Some(f);
                }
            }
        }

        Ok(Demiurge {
            functions,
            main: entry.ok_or(Fault::EntryNotFound)?,
            intern: self.intern,
        })
    }
}

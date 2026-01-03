use crate::cyrene::{Fragment, Instruction, Terminator};
use crate::demiurge::meta::{Lattice, Meta};
use crate::demiurge::Function;
use crate::error::Fault;

impl Function {
    pub fn rewrite(&mut self, meta: &Meta) -> Result<(), Fault> {
        for (_, fragment) in self.dangerous() {
            fragment.rewrite(meta)?;
        }
        Ok(())
    }
}

impl Fragment {
    fn rewrite(&mut self, meta: &Meta) -> Result<(), Fault> {
        let mut new = Vec::new();
        self.phis.retain(|(x, _)| {
            if let Lattice::Const(c) = meta.get(*x) {
                new.push(Instruction::Load(*x, c.clone()));
                return false;
            }
            true
        });
        for instruction in self.instructions.iter_mut() {
            instruction.rewrite(meta)?;
        }
        self.instructions.splice(0..0, new);
        self.terminator.as_mut().unwrap().rewrite(meta)
    }
}

impl Instruction {
    fn rewrite(&mut self, meta: &Meta) -> Result<(), Fault> {
        match self {
            Instruction::Binary(dst, _, _, _) => {
                if let Lattice::Const(c) = meta.get(*dst) {
                    *self = Instruction::Load(*dst, c.clone());
                }
            }
            Instruction::Unary(dst, _, _) => {
                if let Lattice::Const(c) = meta.get(*dst) {
                    *self = Instruction::Load(*dst, c.clone());
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Terminator {
    fn rewrite(&mut self, meta: &Meta) -> Result<(), Fault> {
        if let Terminator::Branch(cond, yes, no) = self
            && let Lattice::Const(c) = meta.get(*cond)
        {
            let label = if c.bool()? { yes } else { no };
            *self = Terminator::Jump(*label);
            return Ok(());
        }
        Ok(())
    }
}

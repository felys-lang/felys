use crate::cyrene::{Fragment, Instruction, Terminator};
use crate::demiurge::context::{Lattice, Meta};
use crate::demiurge::Function;
use crate::error::Fault;

impl Function {
    pub fn rewrite(&mut self, ctx: &Meta) -> Result<(), Fault> {
        for (_, fragment) in self.dangerous() {
            fragment.rewrite(ctx)?;
        }
        Ok(())
    }
}

impl Fragment {
    fn rewrite(&mut self, ctx: &Meta) -> Result<(), Fault> {
        let mut new = Vec::new();
        self.phis.retain(|(x, _)| {
            if let Lattice::Const(c) = ctx.get(*x) {
                new.push(Instruction::Load(*x, c.clone()));
                return false;
            }
            true
        });
        for instruction in self.instructions.iter_mut() {
            instruction.rewrite(ctx)?;
        }
        self.instructions.splice(0..0, new);
        self.terminator.as_mut().unwrap().rewrite(ctx)
    }
}

impl Instruction {
    fn rewrite(&mut self, ctx: &Meta) -> Result<(), Fault> {
        match self {
            Instruction::Binary(dst, _, _, _) => {
                if let Lattice::Const(c) = ctx.get(*dst) {
                    *self = Instruction::Load(*dst, c.clone());
                }
            }
            Instruction::Unary(dst, _, _) => {
                if let Lattice::Const(c) = ctx.get(*dst) {
                    *self = Instruction::Load(*dst, c.clone());
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Terminator {
    fn rewrite(&mut self, ctx: &Meta) -> Result<(), Fault> {
        if let Terminator::Branch(cond, yes, no) = self
            && let Lattice::Const(c) = ctx.get(*cond)
        {
            let label = if c.bool()? { yes } else { no };
            *self = Terminator::Jump(*label);
            return Ok(());
        }
        Ok(())
    }
}

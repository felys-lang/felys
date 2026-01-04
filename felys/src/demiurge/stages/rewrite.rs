use crate::cyrene::{Fragment, Instruction, Terminator};
use crate::demiurge::meta::{Lattice, Meta};
use crate::demiurge::Function;
use crate::error::Fault;

impl Function {
    pub fn rewrite(&mut self, meta: &Meta) -> Result<bool, Fault> {
        let mut changed = false;
        for (_, fragment) in self.dangerous() {
            if fragment.rewrite(meta)? {
                changed = true;
            }
        }
        Ok(changed)
    }
}

impl Fragment {
    fn rewrite(&mut self, meta: &Meta) -> Result<bool, Fault> {
        let mut changed = false;
        let mut new = Vec::new();
        self.phis.retain(|(x, _)| {
            if let Lattice::Const(c) = meta.get(*x) {
                new.push(Instruction::Load(*x, c.clone()));
                changed = true;
                return false;
            }
            true
        });
        for instruction in self.instructions.iter_mut() {
            if instruction.rewrite(meta)? {
                changed = true;
            }
        }
        self.instructions.splice(0..0, new);
        if self.terminator.as_mut().unwrap().rewrite(meta)? {
            changed = true;
        }
        Ok(changed)
    }
}

impl Instruction {
    fn rewrite(&mut self, meta: &Meta) -> Result<bool, Fault> {
        match self {
            Instruction::Binary(dst, _, _, _) | Instruction::Unary(dst, _, _) => {
                if let Lattice::Const(c) = meta.get(*dst) {
                    *self = Instruction::Load(*dst, c.clone());
                    return Ok(true);
                }
            }
            _ => {}
        }
        Ok(false)
    }
}

impl Terminator {
    fn rewrite(&mut self, meta: &Meta) -> Result<bool, Fault> {
        if let Terminator::Branch(cond, yes, no) = self
            && let Lattice::Const(c) = meta.get(*cond)
        {
            let label = if c.bool()? { yes } else { no };
            *self = Terminator::Jump(*label);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

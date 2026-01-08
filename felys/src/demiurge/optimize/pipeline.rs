use crate::cyrene::Function;
use crate::demiurge::Demiurge;
use crate::error::Fault;

impl Demiurge {
    pub fn optimize(mut self, limit: usize) -> Result<Self, Fault> {
        for function in self.fns.values_mut() {
            function.optimize(limit)?;
        }
        self.main.optimize(limit)?;
        Ok(self)
    }
}

impl Function {
    fn optimize(&mut self, limit: usize) -> Result<(), Fault> {
        for _ in 0..limit {
            let mut changed = false;
            let meta = self.analyze()?;

            if self.rewrite(&meta) {
                changed = true;
            }

            if self.rename() {
                changed = true;
            }

            if self.sweep()? {
                changed = true;
            }

            if self.compact() {
                changed = true;
            }

            if !changed {
                break;
            }
        }
        Ok(())
    }
}

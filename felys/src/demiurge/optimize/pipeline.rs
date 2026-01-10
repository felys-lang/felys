use crate::cyrene::Function;
use crate::demiurge::Demiurge;
use crate::error::Fault;

impl Demiurge {
    pub fn optimize(mut self, depth: usize) -> Result<Self, Fault> {
        for function in self.fns.values_mut() {
            function.optimize(depth)?;
        }
        self.main.optimize(depth)?;
        Ok(self)
    }
}

impl Function {
    fn optimize(&mut self, depth: usize) -> Result<(), Fault> {
        for _ in 0..=depth {
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

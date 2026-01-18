use crate::demiurge::fault::Fault;
use crate::demiurge::Demiurge;
use crate::utils::function::Function;

impl Demiurge {
    pub fn optimize(mut self, limit: usize) -> Result<Self, String> {
        for function in self.fns.values_mut() {
            function.optimize(limit).map_err(|e| e.recover())?;
        }
        self.main.optimize(limit).map_err(|e| e.recover())?;
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

            if self.sweep() {
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

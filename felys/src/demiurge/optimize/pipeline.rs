use crate::cyrene::Function;
use crate::demiurge::fault::Fault;
use crate::demiurge::Demiurge;

impl Demiurge {
    pub fn optimize(mut self, additional: usize) -> Result<Self, String> {
        for function in self.fns.values_mut() {
            function.optimize(additional).map_err(|e| e.recover())?;
        }
        self.main.optimize(additional).map_err(|e| e.recover())?;
        Ok(self)
    }
}

impl Function {
    fn optimize(&mut self, additional: usize) -> Result<(), Fault> {
        for _ in 0..additional {
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

use crate::demiurge::error::Error;
use crate::utils::function::Function;

impl Function {
    pub fn optimize(&mut self, limit: usize) -> Result<(), Error> {
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

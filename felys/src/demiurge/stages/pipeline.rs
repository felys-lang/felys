use crate::demiurge::Function;
use crate::error::Fault;

impl Function {
    pub fn optimize(&mut self) -> Result<(), Fault> {
        let mut changed = true;
        while changed {
            changed = false;
            let meta = self.analyze()?;

            if self.prune(&meta) {
                changed = true;
            }

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
        }
        Ok(())
    }
}

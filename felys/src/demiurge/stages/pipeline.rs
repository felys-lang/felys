use crate::demiurge::Function;
use crate::error::Fault;

impl Function {
    pub fn optimize(&mut self) -> Result<(), Fault> {
        let mut changed = true;
        while changed {
            changed = false;
            let meta = self.analyze()?;
            if self.rewrite(&meta)? {
                changed = true;
            }

            if self.rename(&meta) {
                changed = true;
            }

            if self.sweep() {
                changed = true;
            }

            if self.compact() {
                changed = true;
            }
        }
        Ok(())
    }
}

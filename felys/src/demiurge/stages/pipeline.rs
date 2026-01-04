use crate::demiurge::Function;
use crate::error::Fault;

impl Function {
    pub fn optimize(&mut self) -> Result<(), Fault> {
        let mut changed = true;
        while changed {
            changed = false;
            let ctx = self.analyze()?;
            if self.rewrite(&ctx)? {
                changed = true;
            }

            if self.rename(&ctx) {
                changed = true;
            }

            if self.sweep(&ctx) {
                changed = true;
            }
        }
        Ok(())
    }
}

use crate::demiurge::Function;
use crate::error::Fault;

impl Function {
    pub fn optimize(&mut self) -> Result<(), Fault> {
        let ctx = self.analyze()?;
        self.rewrite(&ctx)?;
        self.rename(&ctx);
        self.sweep(&ctx);
        Ok(())
    }
}

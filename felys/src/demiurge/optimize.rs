use crate::demiurge::{Demiurge, Function};
use crate::error::Fault;

impl Demiurge {
    pub fn optimize(mut self) -> Result<Self, Fault> {
        for function in self.fns.values_mut() {
            function.optimize()?;
        }
        self.main.optimize()?;
        Ok(self)
    }
}

impl Function {
    fn optimize(&mut self) -> Result<(), Fault> {
        let ctx = self.analyze()?;
        self.rewrite(&ctx)?;
        self.rename(&ctx);
        self.sweep(&ctx);
        Ok(())
    }
}

use crate::cyrene::Function;
use crate::demiurge::Demiurge;
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
        let mut changed = true;
        while changed {
            changed = false;
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
        }
        Ok(())
    }
}

use crate::demiurge::error::Error;
use crate::utils::function::Function;

impl Function {
    pub fn optimize(&mut self, limit: usize) -> Result<(), Error> {
        for _ in 0..limit {
            let mut changed = false;
            let meta = self.analyze()?;

            changed |= self.rewrite(&meta);
            changed |= self.rename();
            changed |= self.sweep();
            changed |= self.compact();

            if !changed {
                break;
            }
        }
        Ok(())
    }
}

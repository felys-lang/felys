use crate::frontend::cfg::function::Function;
use crate::optimizer::error::Error;

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

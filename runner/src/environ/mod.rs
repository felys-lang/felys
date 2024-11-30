mod value;
mod warehouse;

use packrat::Pool;
use std::collections::HashMap;
pub use value::*;
pub use warehouse::*;

pub struct Environ<'a> {
    pub pool: &'a Pool,
    pub warehouse: Warehouse,
}


impl<'a> Environ<'a> {
    pub fn new(pool: &'a Pool) -> Environ<'a> {
        Self {
            pool,
            warehouse: Warehouse { floors: vec![HashMap::new()] },
        }
    }

    pub fn sandbox(&self) -> Self {
        Self {
            pool: self.pool,
            warehouse: Warehouse { floors: vec![HashMap::new()] },
        }
    }
}
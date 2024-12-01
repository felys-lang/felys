mod value;
mod warehouse;

use packrat::Pool;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
pub use value::*;
pub use warehouse::*;

pub struct Environ<'a> {
    pub pool: &'a Pool,
    pub timer: &'a Receiver<bool>,
    pub depth: (usize, usize),
    pub warehouse: Warehouse,
}


impl<'a> Environ<'a> {
    pub fn new(pool: &'a Pool, timer: &'a Receiver<bool>, depth: usize) -> Environ<'a> {
        Self {
            pool,
            timer,
            depth: (0, depth),
            warehouse: Warehouse { floors: vec![HashMap::new()] },
        }
    }

    pub fn sandbox(&self) -> Self {
        Self {
            pool: self.pool,
            timer: self.timer,
            depth: (self.depth.0 + 1, self.depth.1),
            warehouse: Warehouse { floors: vec![HashMap::new()] },
        }
    }
}
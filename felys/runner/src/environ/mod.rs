mod value;
mod warehouse;

use crate::execute::Signal;
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
    pub fn new(pool: &'a mut Pool, timer: &'a Receiver<bool>, depth: usize) -> Environ<'a> {
        let elysia = "粉色妖精小姐♪".to_string();
        let symbol = pool.id("__elysia__".to_string());
        let ground = HashMap::from([(symbol, Value::Str(elysia))]);
        Self {
            pool,
            timer,
            depth: (0, depth),
            warehouse: Warehouse { floors: vec![ground] },
        }
    }

    pub fn sandbox(&self) -> Result<Self, Signal> {
        if self.depth.0 < self.depth.1 {
            Ok(Self {
                pool: self.pool,
                timer: self.timer,
                depth: (self.depth.0 + 1, self.depth.1),
                warehouse: Warehouse { floors: vec![HashMap::new()] },
            })
        } else {
            Err(Signal::Error("stack overflow"))
        }
    }
}
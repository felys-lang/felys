mod value;
mod warehouse;


use crate::packrat::Intern;
use crate::runner::execute::Signal;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
pub use value::*;
pub use warehouse::*;

pub struct Environ<'a> {
    pub intern: &'a Intern,
    pub timer: &'a Receiver<bool>,
    pub depth: (u64, u64),
    pub warehouse: Warehouse,
}


impl Environ<'_> {
    pub fn sandbox(&self) -> Result<Self, Signal> {
        if self.depth.0 < self.depth.1 {
            Ok(Self {
                intern: self.intern,
                timer: self.timer,
                depth: (self.depth.0 + 1, self.depth.1),
                warehouse: Warehouse { floors: vec![HashMap::new()] },
            })
        } else {
            Err(Signal::Error("stack overflow"))
        }
    }
}
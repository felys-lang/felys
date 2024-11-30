mod value;
mod warehouse;

use packrat::Pool;
pub use value::*;
pub use warehouse::*;

pub struct Environ {
    pub writer: Writer,
    pub pool: Pool,
    pub warehouse: Warehouse,
}

pub struct Writer {
    pub buffer: String,
}
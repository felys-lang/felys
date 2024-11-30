mod value;

use packrat::Pool;
pub use value::*;

pub struct Environ {
    pub writer: Writer,
    pub pool: Pool,
}

pub struct Writer {
    pub buffer: String,
}

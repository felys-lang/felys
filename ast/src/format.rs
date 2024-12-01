use std::fmt::Formatter;

pub const INDENT: &str = "    ";

pub trait Indenter {
    fn print(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result;
}
use crate::cyrene::{Const, Group};
use crate::demiurge::Bytecode;

pub struct Elysia {
    pub main: Vec<Bytecode>,
    pub text: Vec<Vec<Bytecode>>,
    pub data: Vec<Const>,
    pub lookup: Vec<Group>,
}

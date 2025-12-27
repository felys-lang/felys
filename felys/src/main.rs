use crate::error::Fault;
use crate::philia093::PhiLia093;

mod ast;
mod cyrene;
mod demiurge;
mod error;
mod philia093;
mod elysia;

const CODE: &str = r#"
fn x() {
    a = 1;
    b = 1;
    c = 1;
    return 1;
}

fn y() {
    return 1;
}

fn z() {
    return 1;
}

fn main(args) {
    z(1, 2, x(3, 4, 5))
}
"#;

fn main() -> Result<(), Fault> {
    let philia093 = PhiLia093::from(CODE.to_string());
    let cyrene = philia093.parse()?;
    let demiurge = cyrene.cfg()?;
    println!("{:?}", demiurge.functions);
    println!("{:?}", demiurge.main);
    Ok(())
}

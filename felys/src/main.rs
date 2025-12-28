use crate::error::Fault;
use crate::philia093::PhiLia093;

mod ast;
mod cyrene;
mod demiurge;
mod elysia;
mod error;
mod philia093;

const CODE: &str = r#"
group Test(a);

impl Test {
    fn x() {
        a = 1;
        b = 1;
        c = 1;
        a += c;
        return 1;
    }

    fn y(self) {
        return self + 1;
    }
}

fn y() {
    return 1;
}

fn z() {
    return 1;
}

fn main(args) {
    Test(1, 2, z(3, 4, 5)).y().a
}
"#;

fn main() -> Result<(), Fault> {
    let philia093 = PhiLia093::from(CODE.to_string());
    let cyrene = philia093.parse()?;
    let demiurge = cyrene.cfg()?;
    let elysia = demiurge.codegen()?;
    println!("{:?}", elysia.main);
    println!("{:?}", elysia.text);
    println!("{:?}", elysia.data);
    println!("{:?}", elysia.lookup);
    Ok(())
}

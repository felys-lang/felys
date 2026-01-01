use crate::error::Fault;
use crate::philia093::PhiLia093;

mod ast;
mod cyrene;
mod demiurge;
mod elysia;
mod error;
mod philia093;

const CODE: &str = r#"
fn main(args) {
    i = 0;
    if true {
        if false {
            return 3;
        }
        i = 1;
    } else {
        i = 2;
    }
    return i;
}
"#;

fn main() -> Result<(), Fault> {
    let philia093 = PhiLia093::from(CODE.to_string());
    let cyrene = philia093.parse()?;
    let demiurge = cyrene.cfg()?;
    println!("{:?}", demiurge.main);
    // let elysia = demiurge.codegen()?;
    // println!("{:?}", elysia.main);
    // println!("{:?}", elysia.text);
    // println!("{:?}", elysia.data);
    // println!("{:?}", elysia.lookup);
    Ok(())
}

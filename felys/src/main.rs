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
    x = 1;
    x = 2;
    if false {
        x = 3;
    }
    0
}
"#;

fn main() -> Result<(), Fault> {
    let philia093 = PhiLia093::from(CODE.to_string());
    let cyrene = philia093.parse()?;
    let demiurge = cyrene.cfg()?.optimize()?;
    println!("{:?}", demiurge.main.entry);
    for frag in demiurge.main.fragments {
        println!("{:?}", frag);
    }
    println!("{:?}", demiurge.main.exit);
    Ok(())
}

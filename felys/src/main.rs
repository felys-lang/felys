use crate::error::Fault;
use crate::philia093::PhiLia093;

mod ast;
mod cyrene;
mod demiurge;
mod error;
mod philia093;

const CODE: &str = r#"
fn main(args) {
    loop { break args; }
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

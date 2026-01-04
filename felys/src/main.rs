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
    if true {
        if true {
            if true {
                if true {
                    if true {
                        return args;
                    }
                }
            }
        }
    }
    0
}
"#;

fn main() -> Result<(), Fault> {
    let philia093 = PhiLia093::from(CODE.to_string());
    let cyrene = philia093.parse()?;
    let demiurge = cyrene.cfg()?.optimize()?;
    println!("{:?}", demiurge.main.entry);
    println!("{:?}", demiurge.main.fragments);
    println!("{:?}", demiurge.main.exit);
    Ok(())
}

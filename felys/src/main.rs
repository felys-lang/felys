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
    x = x + 2;
    x = x;
    if false {
        // 这部分代码不可达
        y = 10; // y 应该是 Top
        z = y + 1; // z 应该是 Top
    }
    x
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

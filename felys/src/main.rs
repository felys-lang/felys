use crate::ast::Item;
use crate::cyrene::{Context, Function};
use crate::philia093::PhiLia093;

mod ast;
mod cyrene;
mod philia093;
mod error;

const CODE: &str = r#"
fn main(args) {
     std::b;
}
"#;

fn main() {
    let mut philia093 = PhiLia093::from(CODE.to_string());
    let root = philia093.root();
    println!("{:?}", philia093.__snapshot);
    println!("{:?}", root);
    // for item in root.unwrap().0.iter() {
    //     let block = match item {
    //         Item::Fn(_, _, x) => x,
    //         Item::Main(_, x) => x,
    //         _ => continue,
    //     };
    //     let mut f = Function::new();
    //     let mut ctx = Context::new();
    //     block.ir(&mut f, &mut ctx);
    //     println!("{:?}", f.segments);
    // }
}

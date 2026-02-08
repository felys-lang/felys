use crate::philia093::PhiLia093;
use builder::common::Builder;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

mod ast;
mod builder;
mod philia093;
mod utils;

#[derive(Parser)]
struct Args {
    /// specify the grammar file fed to the generator
    grammar: PathBuf,
    /// target directory of generated code
    target: PathBuf,
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(&args.grammar).expect("file not found");
    let now = Instant::now();
    let (grammar, intern) = match PhiLia093::from(code).parse() {
        Ok(inner) => inner,
        Err(e) => return println!("{}", e),
    };
    Builder::new(grammar, intern)
        .codegen()
        .write(&args.target, "philia093");
    println!("elapsed: {:?}", now.elapsed());
}

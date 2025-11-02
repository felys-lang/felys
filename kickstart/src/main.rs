use crate::parser::Packrat;
use builder::common::Builder;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod ast;
mod builder;
mod parser;
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
    let mut packrat = Packrat::from(code);
    let grammar = packrat.grammar();

    if let Some((cursor, msg)) = &packrat.snapshot {
        let data = &packrat.stream.data;
        let before = data[..*cursor]
            .rfind('\n')
            .map_or(&data[..*cursor], |n| &data[n + 1..*cursor]);
        let after = data[*cursor..]
            .find('\n')
            .map_or(&data[*cursor..], |n| &data[*cursor..*cursor + n]);

        let x = before.chars().count();
        let y = data[..*cursor].chars().filter(|c| *c == '\n').count() + 1;

        println!("Error @ {}:{}:{}", args.grammar.to_str().unwrap(), x, y);
        println!("{}{}", before, after);
        println!("{}^", " ".repeat(x));
        println!("Hint: {}", msg);
        return;
    }

    Builder::new(grammar.unwrap(), packrat.intern)
        .codegen()
        .write(&args.target, "parser");
}

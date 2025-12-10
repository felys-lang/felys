use crate::philia093::PhiLia093;
use builder::common::Builder;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

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
    let mut packrat = PhiLia093::from(code);
    let grammar = packrat.grammar();

    if let Some((cursor, msg)) = &packrat.__snapshot {
        let data = &packrat.__stream.data;
        let before = data[..*cursor]
            .rfind('\n')
            .map_or(&data[..*cursor], |n| &data[n + 1..*cursor]);
        let after = data[*cursor..]
            .find('\n')
            .map_or(&data[*cursor..], |n| &data[*cursor..*cursor + n]);

        let x = before.chars().count();
        let y = data[..*cursor].chars().filter(|c| *c == '\n').count() + 1;
        let padding = y.to_string().len();

        println!("error: {}", msg);
        println!(
            "{}--> {}:{}:{}",
            " ".repeat(padding),
            args.grammar.to_str().unwrap(),
            y,
            x
        );
        println!("{} |", " ".repeat(padding));
        println!("{} | {}{}", y, before, after);
        println!("{} | {}^", " ".repeat(padding), " ".repeat(x));

        return;
    }

    Builder::new(grammar.unwrap(), packrat.__intern)
        .codegen()
        .write(&args.target, "philia093");
}

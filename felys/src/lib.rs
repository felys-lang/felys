mod ast;
mod rspegen;
mod runtime;

#[cfg(test)]
mod tests {
    use crate::rspegen::Packrat;

    #[test]
    fn playground() {
        let mut packrat = Packrat::from("(y,a) = 1.0;".to_string());
        match packrat.parse() {
            Ok(ast) => println!("{:?}", ast),
            Err(msg) => println!("Error: {}", msg),
        }
    }
}

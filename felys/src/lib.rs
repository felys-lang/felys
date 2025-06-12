mod ast;
mod rspegen;
mod runtime;

#[cfg(test)]
mod tests {
    use crate::rspegen::Packrat;

    #[test]
    fn playground() {
        let mut packrat = Packrat::from("a = 1.0; return a;".to_string());
        let ast = match packrat.parse() {
            Ok(ast) => ast,
            Err(msg) => {
                println!("Error: {}", msg);
                return;
            }
        };
        match ast.exec(packrat.intern, 100, 1000) {
            Ok(value) => println!("{}", value),
            Err(msg) => println!("Error: {}", msg),
        }
    }
}

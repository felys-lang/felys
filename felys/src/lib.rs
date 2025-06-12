mod ast;
mod rspegen;

#[cfg(test)]
mod tests {
    use crate::rspegen::Packrat;

    #[test]
    fn playground() {
        let mut packrat = Packrat::from("let x = 1.0".to_string());
        let result = packrat.grammar();
        if let Some((loc, msg)) = packrat.snapshot {
            println!("{} @ {}", msg, loc);
        }
        println!("{:?}", result);
    }
}

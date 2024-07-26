#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    use felys::*;

    fn execute(filename: &str) -> Object {
        let file = ["tests", "programs", filename].iter().collect::<PathBuf>();
        let code = read_to_string(file).unwrap();
        let mixin = HashMap::new();
        let mut main = Worker::new(mixin, 0.0, Language::EN);
        match main.exec(code) {
            Ok(s) => s.code,
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn felys() {
        assert_eq!(
            execute("felys.ely"),
            Object::String { value: "爱莉希雅".into() }
        );
    }

    #[test]
    fn factorial() {
        assert_eq!(
            execute("factorial.ely"),
            Object::Number { value: 3628800.0 }
        );
    }
}

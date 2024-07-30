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
            Ok(s) => s.exit,
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn felys() {
        assert_eq!(
            execute("felys.ely"),
            Object::String("爱莉希雅".into())
        );
    }

    #[test]
    fn factorial() {
        assert_eq!(
            execute("factorial.ely"),
            Object::Number(3628800.0)
        );
    }

    #[test]
    fn heaviside() {
        assert_eq!(
            execute("heaviside.ely"),
            Object::Number(0.5)
        );
    }
}

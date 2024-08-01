#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    use felys::*;
    use felys::Language::*;

    fn execute(filename: &str, lang: Language) -> Object {
        let file = ["tests", "programs", filename].iter().collect::<PathBuf>();
        let code = read_to_string(file).unwrap();
        let mixin = HashMap::new();
        let mut main = Worker::new(mixin, 0.0, lang);
        match main.exec(code) {
            Ok(s) => s.exit,
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn felys() {
        assert_eq!(
            execute("felys.ely", EN),
            Object::String("爱莉希雅".into())
        );
    }

    #[test]
    fn chinese() {
        assert_eq!(
            execute("chinese.ely", ZH),
            Object::String("粉色妖精小姐♪".into())
        );
    }

    #[test]
    fn factorial() {
        assert_eq!(
            execute("factorial.ely", EN),
            Object::Number(3628800.0)
        );
    }

    #[test]
    fn heaviside() {
        assert_eq!(
            execute("heaviside.ely", EN),
            Object::Number(0.5)
        );
    }
}

use felys::Packrat;
use std::collections::HashMap;

#[test]
fn builtin() {
    assert_eq!("42", run("42"));
    assert_eq!("3.14", run("3.14"));
    assert_eq!("true", run("true"));
    assert_eq!("false", run("false"));
    assert_eq!(r#"hello"#, run(r#""hello""#));
    assert_eq!("(1, 2, 3)", run("(1, 2, 3)"));
    assert_eq!("[1, 2, 3]", run("[1, 2, 3]"));
}

#[test]
fn arithmetic() {
    assert_eq!("5", run("2 + 3"));
    assert_eq!("1", run("4 - 3"));
    assert_eq!("6", run("3 * 2"));
    assert_eq!("2", run("8 / 4"));
    assert_eq!("1", run("10 % 3"));
}

#[test]
fn comparison() {
    assert_eq!("true", run("1 == 1"));
    assert_eq!("false", run("1 == 2"));
    assert_eq!("true", run("1 != 2"));
    assert_eq!("false", run("2 != 2"));
    assert_eq!("true", run("3 > 2"));
    assert_eq!("false", run("2 > 3"));
    assert_eq!("true", run("3 >= 3"));
    assert_eq!("false", run("2 >= 3"));
    assert_eq!("true", run("2 < 3"));
    assert_eq!("false", run("3 < 2"));
    assert_eq!("true", run("3 <= 3"));
    assert_eq!("false", run("4 <= 3"));
}

#[test]
fn logical() {
    assert_eq!("false", run("false and true"));
    assert_eq!("true", run("true or false"));
    assert_eq!("true", run("not false"));
    assert_eq!("false", run("not true"));
    assert_eq!("true", run("true and not false"));
    assert_eq!("false", run("false or false and true"));
    assert_eq!("true", run("not false or false"));
}

#[test]
fn nn() {
    assert_eq!(
        "[\n  1.000 1.000 ;\n  1.000 1.000 ;\n](backward: Fixed)",
        run("[1.0, 1.0; 1.0, 1.0;]")
    );
    assert_eq!(
        "[\n  -0.495 -0.824 ;\n  0.155 -0.555 ;\n](backward: Learnable(14))",
        run("<2, 2>")
    );
    assert_eq!("[\n  0.495 ;\n](backward: Neg)", run("-<1, 1>"));
    assert_eq!("[\n  -1.319 ;\n](backward: Add)", run("<1, 1> + <1, 1>"));
    assert_eq!("[\n  0.328 ;\n](backward: Sub)", run("<1, 1> - <1, 1>"));
    assert_eq!("[\n  0.408 ;\n](backward: Mul)", run("<1, 1> * <1, 1>"));
    assert_eq!("[\n  0.601 ;\n](backward: Div)", run("<1, 1> / <1, 1>"));
    assert_eq!("[\n  0.381 ;\n](backward: Dot)", run("<1, 2> @ <2, 1>"));
    assert_eq!(
        "[\n  0.000 0.000 0.155 ;\n](backward: ReLU)",
        run("rust ReLU(<1, 3>)")
    );
    assert_eq!(
        "[\n  1.619 ;\n](backward: CrossEntropy)",
        run("rust CrossEntropy(<1, 3>, [0.0, 1.0, 0.0;])")
    );
    assert_eq!("<void>", run("step [1.0;] by 0.01"));
}

#[test]
fn combined() {
    assert_eq!("6", run("1 + 2 + 3"));
    assert_eq!("7", run("1 + 2 * 3"));
    assert_eq!("9", run("(1 + 2) * 3"));
    assert_eq!("true", run("1 + 2 == 3"));
    assert_eq!("1", run("if true { 1 } else { 2 }"));
    assert_eq!("10", run("a = 0; while a < 10 { a += 1 }; a"));
    assert_eq!("6", run("sum = 0; for x in [1, 2, 3] { sum += x }; sum"));
    assert_eq!("2", run("i = 0; loop { i += 1; if i == 2 { break }; }; i"));
    assert_eq!("5", run("f = |x| x + 2; f(3)"));
    assert_eq!("7", run("{ a = 2; { b = 5; a + b } }"));
}

fn run(code: &str) -> String {
    let wrapped = format!("print {{ {code} }};");
    let params = HashMap::new();
    let executable = match Packrat::from(wrapped).parse() {
        Ok(program) => program.config(params, 100, 100, 0.9, 42),
        Err(msg) => return msg,
    };
    match executable.exec() {
        Ok(output) => output.stdout.first().unwrap().to_string(),
        Err(msg) => msg,
    }
}

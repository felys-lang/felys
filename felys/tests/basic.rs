use felys::{Config, Packrat};
use std::collections::HashMap;

#[test]
fn builtin() -> Result<(), String> {
    assert_eq!("42", print("42")?);
    assert_eq!("3.14", print("3.14")?);
    assert_eq!("true", print("true")?);
    assert_eq!("false", print("false")?);
    assert_eq!(r#"hello"#, print(r#""hello""#)?);
    assert_eq!("(1, 2, 3)", print("(1, 2, 3)")?);
    assert_eq!("[1, 2, 3]", print("[1, 2, 3]")?);
    Ok(())
}

#[test]
fn arithmetic() -> Result<(), String> {
    assert_eq!("5", print("2 + 3")?);
    assert_eq!("1", print("4 - 3")?);
    assert_eq!("6", print("3 * 2")?);
    assert_eq!("2", print("8 / 4")?);
    assert_eq!("1", print("10 % 3")?);
    Ok(())
}

#[test]
fn comparison() -> Result<(), String> {
    assert_eq!("true", print("1 == 1")?);
    assert_eq!("false", print("1 == 2")?);
    assert_eq!("true", print("1 != 2")?);
    assert_eq!("false", print("2 != 2")?);
    assert_eq!("true", print("3 > 2")?);
    assert_eq!("false", print("2 > 3")?);
    assert_eq!("true", print("3 >= 3")?);
    assert_eq!("false", print("2 >= 3")?);
    assert_eq!("true", print("2 < 3")?);
    assert_eq!("false", print("3 < 2")?);
    assert_eq!("true", print("3 <= 3")?);
    assert_eq!("false", print("4 <= 3")?);
    Ok(())
}

#[test]
fn logical() -> Result<(), String> {
    assert_eq!("false", print("false and true")?);
    assert_eq!("true", print("true or false")?);
    assert_eq!("true", print("not false")?);
    assert_eq!("false", print("not true")?);
    assert_eq!("true", print("true and not false")?);
    assert_eq!("false", print("false or false and true")?);
    assert_eq!("true", print("not false or false")?);
    Ok(())
}

#[test]
fn combined() -> Result<(), String> {
    assert_eq!("6", print("1 + 2 + 3")?);
    assert_eq!("7", print("1 + 2 * 3")?);
    assert_eq!("9", print("(1 + 2) * 3")?);
    assert_eq!("true", print("1 + 2 == 3")?);
    assert_eq!("1", print("if true { 1 } else { 2 }")?);
    assert_eq!("10", print("a = 0; while a < 10 { a += 1 }; a")?);
    assert_eq!("6", print("sum = 0; for x in [1, 2, 3] { sum += x }; sum")?);
    assert_eq!(
        "2",
        print("i = 0; loop { i += 1; if i == 2 { break }; }; i")?
    );
    assert_eq!("5", print("f = |x| x + 2; f(3)")?);
    assert_eq!("7", print("{ a = 2; { b = 5; a + b } }")?);
    Ok(())
}

fn print(code: &str) -> Result<String, String> {
    let wrapped = format!("print {{ {code} }};");
    let params = HashMap::new();
    let config = Config::new(params, 100, 100, 0.9, 42);
    let output = Packrat::from(wrapped).parse()?.config(config).exec()?;
    Ok(output.stdout.join("\n"))
}

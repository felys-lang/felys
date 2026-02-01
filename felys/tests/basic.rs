use crate::utils::exec;
use felys::Object;

mod utils;

#[test]
fn object() -> Result<(), String> {
    let args = Object::List([].into());

    exec(args.clone(), "", "42", Object::Int(42), "")?;
    exec(args.clone(), "", "9.8", Object::Float(9.8), "")?;
    exec(args.clone(), "", "true", Object::Bool(true), "")?;
    exec(
        args.clone(),
        "",
        "\"你好，世界！\"",
        Object::Str("你好，世界！".into()),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "\"hello, world!\"",
        Object::Str("hello, world!".into()),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "[0, [0, 0]]",
        Object::List(
            [
                Object::Int(0),
                Object::List([Object::Int(0), Object::Int(0)].into()),
            ]
            .into(),
        ),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "(1, (1, 1))",
        Object::Tuple(
            [
                Object::Int(1),
                Object::Tuple([Object::Int(1), Object::Int(1)].into()),
            ]
            .into(),
        ),
        "",
    )?;

    Ok(())
}

#[test]
fn arithmetic() -> Result<(), String> {
    let args = Object::List([].into());

    exec(args.clone(), "", "2 + 3", Object::Int(5), "")?;
    exec(args.clone(), "", "2 - 3", Object::Int(-1), "")?;
    exec(args.clone(), "", "2 * 3", Object::Int(6), "")?;
    exec(args.clone(), "", "2 / 3", Object::Int(0), "")?;
    exec(args.clone(), "", "2 % 3", Object::Int(2), "")?;
    exec(args.clone(), "", "2.0 + 3.0", Object::Float(5.0), "")?;
    exec(args.clone(), "", "2.0 - 3.0", Object::Float(-1.0), "")?;
    exec(args.clone(), "", "2.0 * 3.0", Object::Float(6.0), "")?;
    exec(args.clone(), "", "3.0 / 2.0", Object::Float(1.5), "")?;
    exec(args.clone(), "", "+2", Object::Int(2), "")?;
    exec(args.clone(), "", "-2", Object::Int(-2), "")?;
    exec(args.clone(), "", "+2.0", Object::Float(2.0), "")?;
    exec(args.clone(), "", "-2.0", Object::Float(-2.0), "")?;

    Ok(())
}

#[test]
fn logical() -> Result<(), String> {
    let args = Object::List([].into());

    exec(args.clone(), "", "true and true", Object::Bool(true), "")?;
    exec(args.clone(), "", "true and false", Object::Bool(false), "")?;
    exec(args.clone(), "", "false and true", Object::Bool(false), "")?;
    exec(args.clone(), "", "false and false", Object::Bool(false), "")?;
    exec(args.clone(), "", "true or true", Object::Bool(true), "")?;
    exec(args.clone(), "", "true or false", Object::Bool(true), "")?;
    exec(args.clone(), "", "false or true", Object::Bool(true), "")?;
    exec(args.clone(), "", "false or false", Object::Bool(false), "")?;
    exec(args.clone(), "", "not true", Object::Bool(false), "")?;
    exec(args.clone(), "", "not false", Object::Bool(true), "")?;

    Ok(())
}

#[test]
fn comparison() -> Result<(), String> {
    let args = Object::List([].into());

    exec(args.clone(), "", "1 > 1", Object::Bool(false), "")?;
    exec(args.clone(), "", "2 > 1", Object::Bool(true), "")?;
    exec(args.clone(), "", "1 >= 1", Object::Bool(true), "")?;
    exec(args.clone(), "", "1 < 1", Object::Bool(false), "")?;
    exec(args.clone(), "", "1 < 2", Object::Bool(true), "")?;
    exec(args.clone(), "", "1 <= 1", Object::Bool(true), "")?;
    exec(args.clone(), "", "1 == 1", Object::Bool(true), "")?;
    exec(args.clone(), "", "1 == 2", Object::Bool(false), "")?;
    exec(args.clone(), "", "1 != 1", Object::Bool(false), "")?;
    exec(args.clone(), "", "1 != 2", Object::Bool(true), "")?;

    Ok(())
}

#[test]
fn condition() -> Result<(), String> {
    let args = Object::List([].into());

    exec(
        args.clone(),
        "",
        "if true { 1 } else { 0 }",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "if true { 1 } else { return 0; }",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "if true { return 1; } else { 0 }",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = false; if true { x = 1; } else { return 0; } x",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = false; if true { return 1; } else { x = 0; } x",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "if true { return 1; } else { return 0; }",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "while true { if true { return 1; } else { return 0; } } 0",
        Object::Int(1),
        "",
    )?;
    exec(
        Object::Bool(true),
        "",
        "while args { if true { break; } else { break; } } 1",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "if true { if true { if true { return 1; } } } args",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "if true { if true { if true { return args; } } } 0",
        args.clone(),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "if true { if true { if false { return args; } } } 1",
        Object::Int(1),
        "",
    )?;

    Ok(())
}

#[test]
fn loops() -> Result<(), String> {
    let args = Object::List([].into());

    exec(args.clone(), "", "loop { break 1; }", Object::Int(1), "")?;
    exec(args.clone(), "", "loop { return 1; }", Object::Int(1), "")?;
    exec(
        args.clone(),
        "",
        "x = true; loop { if x { x = false; continue; } else { break 1; } }",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "loop { break loop { break loop { break 1; } } }",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 0; while x < 10 { x += 1; } x",
        Object::Int(10),
        "",
    )?;

    Ok(())
}

#[test]
fn functions() -> Result<(), String> {
    let args = Object::List([].into());

    exec(args.clone(), "fn one() { 1 }", "one()", Object::Int(1), "")?;
    exec(
        args.clone(),
        "fn a(a, b, c) { a }",
        "a(1, 2, 3)",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "fn b(a, b, c) { b }",
        "b(1, 2, 3)",
        Object::Int(2),
        "",
    )?;
    exec(
        args.clone(),
        "fn c(a, b, c) { c }",
        "c(1, 2, 3)",
        Object::Int(3),
        "",
    )?;
    exec(
        args.clone(),
        "fn fib(n) { if n <= 1 { n } else { fib(n - 1) + fib(n - 2) } }",
        "fib(10)",
        Object::Int(55),
        "",
    )?;

    Ok(())
}

use crate::utils::eval;
use felys::Object;

mod utils;

#[test]
fn object() -> Result<(), String> {
    let args = Object::List([].into());

    eval(args.clone(), "42", Object::Int(42), "")?;
    eval(args.clone(), "9.8", Object::Float(9.8), "")?;
    eval(args.clone(), "true", Object::Bool(true), "")?;
    eval(
        args.clone(),
        "\"你好，世界！\"",
        Object::Str("你好，世界！".into()),
        "",
    )?;
    eval(
        args.clone(),
        "\"hello, world!\"",
        Object::Str("hello, world!".into()),
        "",
    )?;
    eval(
        args.clone(),
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
    eval(
        args.clone(),
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

    eval(args.clone(), "2 + 3", Object::Int(5), "")?;
    eval(args.clone(), "2 - 3", Object::Int(-1), "")?;
    eval(args.clone(), "2 * 3", Object::Int(6), "")?;
    eval(args.clone(), "2 / 3", Object::Int(0), "")?;
    eval(args.clone(), "2 % 3", Object::Int(2), "")?;
    eval(args.clone(), "2.0 + 3.0", Object::Float(5.0), "")?;
    eval(args.clone(), "2.0 - 3.0", Object::Float(-1.0), "")?;
    eval(args.clone(), "2.0 * 3.0", Object::Float(6.0), "")?;
    eval(args.clone(), "3.0 / 2.0", Object::Float(1.5), "")?;
    eval(args.clone(), "+2", Object::Int(2), "")?;
    eval(args.clone(), "-2", Object::Int(-2), "")?;
    eval(args.clone(), "+2.0", Object::Float(2.0), "")?;
    eval(args.clone(), "-2.0", Object::Float(-2.0), "")?;
    Ok(())
}

#[test]
fn logical() -> Result<(), String> {
    let args = Object::List([].into());

    eval(args.clone(), "true and true", Object::Bool(true), "")?;
    eval(args.clone(), "true and false", Object::Bool(false), "")?;
    eval(args.clone(), "false and true", Object::Bool(false), "")?;
    eval(args.clone(), "false and false", Object::Bool(false), "")?;
    eval(args.clone(), "true or true", Object::Bool(true), "")?;
    eval(args.clone(), "true or false", Object::Bool(true), "")?;
    eval(args.clone(), "false or true", Object::Bool(true), "")?;
    eval(args.clone(), "false or false", Object::Bool(false), "")?;
    eval(args.clone(), "not true", Object::Bool(false), "")?;
    eval(args.clone(), "not false", Object::Bool(true), "")?;

    Ok(())
}

#[test]
fn comparison() -> Result<(), String> {
    let args = Object::List([].into());

    eval(args.clone(), "1 > 1", Object::Bool(false), "")?;
    eval(args.clone(), "2 > 1", Object::Bool(true), "")?;
    eval(args.clone(), "1 >= 1", Object::Bool(true), "")?;
    eval(args.clone(), "1 < 1", Object::Bool(false), "")?;
    eval(args.clone(), "1 < 2", Object::Bool(true), "")?;
    eval(args.clone(), "1 <= 1", Object::Bool(true), "")?;
    eval(args.clone(), "1 == 1", Object::Bool(true), "")?;
    eval(args.clone(), "1 == 2", Object::Bool(false), "")?;
    eval(args.clone(), "1 != 1", Object::Bool(false), "")?;
    eval(args.clone(), "1 != 2", Object::Bool(true), "")?;

    Ok(())
}

#[test]
fn condition() -> Result<(), String> {
    let args = Object::List([].into());

    eval(args.clone(), "if true { 1 } else { 0 }", Object::Int(1), "")?;
    eval(
        args.clone(),
        "if true { 1 } else { return 0; }",
        Object::Int(1),
        "",
    )?;
    eval(
        args.clone(),
        "if true { return 1; } else { 0 }",
        Object::Int(1),
        "",
    )?;
    eval(
        args.clone(),
        "x = false; if true { x = 1; } else { return 0; } x",
        Object::Int(1),
        "",
    )?;
    eval(
        args.clone(),
        "x = false; if true { return 1; } else { x = 0; } x",
        Object::Int(1),
        "",
    )?;
    eval(
        args.clone(),
        "if true { return 1; } else { return 0; }",
        Object::Int(1),
        "",
    )?;
    eval(
        args.clone(),
        "while true { if true { return 1; } else { return 0; } } 0",
        Object::Int(1),
        "",
    )?;
    eval(
        Object::Bool(true),
        "while args { if true { break; } else { break; } } 1",
        Object::Int(1),
        "",
    )?;
    eval(
        args.clone(),
        "if true { if true { if true { return 1; } } } args",
        Object::Int(1),
        "",
    )?;
    eval(
        args.clone(),
        "if true { if true { if true { return args; } } } 0",
        args.clone(),
        "",
    )?;
    eval(
        args.clone(),
        "if true { if true { if false { return args; } } } 1",
        Object::Int(1),
        "",
    )?;

    Ok(())
}

#[test]
fn loops() -> Result<(), String> {
    let args = Object::List([].into());

    eval(args.clone(), "loop { break 1; }", Object::Int(1), "")?;
    eval(args.clone(), "loop { return 1; }", Object::Int(1), "")?;
    eval(
        args.clone(),
        "x = true; loop { if x { x = false; continue; } else { break 1; } }",
        Object::Int(1),
        "",
    )?;
    eval(
        args.clone(),
        "loop { break loop { break loop { break 1; } } }",
        Object::Int(1),
        "",
    )?;

    Ok(())
}

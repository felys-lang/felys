use crate::utils::{eq, eval};
use felys::Object;

mod utils;

#[test]
fn object() -> Result<(), String> {
    let args = Object::List([].into());

    let (_, exit) = eval(args.clone(), "42")?;
    assert!(eq(exit, Object::Int(42))?);

    let (_, exit) = eval(args.clone(), "9.8")?;
    assert!(eq(exit, Object::Float(9.8))?);

    let (_, exit) = eval(args.clone(), "true")?;
    assert!(eq(exit, Object::Bool(true))?);

    let (_, exit) = eval(args.clone(), "\"你好，世界！\"")?;
    assert!(eq(exit, Object::Str("你好，世界！".into()))?);

    let (_, exit) = eval(args.clone(), "\"hello, world!\"")?;
    assert!(eq(exit, Object::Str("hello, world!".into()))?);

    let (_, exit) = eval(args.clone(), "[0, [0, 0]]")?;
    assert!(eq(
        exit,
        Object::List(
            [
                Object::Int(0),
                Object::List([Object::Int(0), Object::Int(0)].into())
            ]
            .into()
        )
    )?);

    let (_, exit) = eval(args.clone(), "(1, (1, 1))")?;
    assert!(eq(
        exit,
        Object::Tuple(
            [
                Object::Int(1),
                Object::Tuple([Object::Int(1), Object::Int(1)].into())
            ]
            .into()
        )
    )?);

    Ok(())
}

#[test]
fn arithmetic() -> Result<(), String> {
    let args = Object::List([].into());

    let (_, exit) = eval(args.clone(), "2 + 3")?;
    assert!(eq(exit, Object::Int(5))?);

    let (_, exit) = eval(args.clone(), "2 - 3")?;
    assert!(eq(exit, Object::Int(-1))?);

    let (_, exit) = eval(args.clone(), "2 * 3")?;
    assert!(eq(exit, Object::Int(6))?);

    let (_, exit) = eval(args.clone(), "2 / 3")?;
    assert!(eq(exit, Object::Int(0))?);

    let (_, exit) = eval(args.clone(), "2 % 3")?;
    assert!(eq(exit, Object::Int(2))?);

    let (_, exit) = eval(args.clone(), "2.0 + 3.0")?;
    assert!(eq(exit, Object::Float(5.0))?);

    let (_, exit) = eval(args.clone(), "2.0 - 3.0")?;
    assert!(eq(exit, Object::Float(-1.0))?);

    let (_, exit) = eval(args.clone(), "2.0 * 3.0")?;
    assert!(eq(exit, Object::Float(6.0))?);

    let (_, exit) = eval(args.clone(), "3.0 / 2.0")?;
    assert!(eq(exit, Object::Float(1.5))?);

    let (_, exit) = eval(args.clone(), "+2")?;
    assert!(eq(exit, Object::Int(2))?);

    let (_, exit) = eval(args.clone(), "-2")?;
    assert!(eq(exit, Object::Int(-2))?);

    let (_, exit) = eval(args.clone(), "+2.0")?;
    assert!(eq(exit, Object::Float(2.0))?);

    let (_, exit) = eval(args.clone(), "-2.0")?;
    assert!(eq(exit, Object::Float(-2.0))?);

    Ok(())
}

#[test]
fn logical() -> Result<(), String> {
    let args = Object::List([].into());

    let (_, exit) = eval(args.clone(), "true and true")?;
    assert!(eq(exit, Object::Bool(true))?);

    let (_, exit) = eval(args.clone(), "true and false")?;
    assert!(eq(exit, Object::Bool(false))?);

    let (_, exit) = eval(args.clone(), "false and true")?;
    assert!(eq(exit, Object::Bool(false))?);

    let (_, exit) = eval(args.clone(), "false and false")?;
    assert!(eq(exit, Object::Bool(false))?);

    let (_, exit) = eval(args.clone(), "true or true")?;
    assert!(eq(exit, Object::Bool(true))?);

    let (_, exit) = eval(args.clone(), "true or false")?;
    assert!(eq(exit, Object::Bool(true))?);

    let (_, exit) = eval(args.clone(), "false or true")?;
    assert!(eq(exit, Object::Bool(true))?);

    let (_, exit) = eval(args.clone(), "false or false")?;
    assert!(eq(exit, Object::Bool(false))?);

    let (_, exit) = eval(args.clone(), "not true")?;
    assert!(eq(exit, Object::Bool(false))?);

    let (_, exit) = eval(args.clone(), "not false")?;
    assert!(eq(exit, Object::Bool(true))?);

    Ok(())
}

#[test]
fn comparison() -> Result<(), String> {
    let args = Object::List([].into());

    let (_, exit) = eval(args.clone(), "1 > 1")?;
    assert!(eq(exit, Object::Bool(false))?);

    let (_, exit) = eval(args.clone(), "2 > 1")?;
    assert!(eq(exit, Object::Bool(true))?);

    let (_, exit) = eval(args.clone(), "1 >= 1")?;
    assert!(eq(exit, Object::Bool(true))?);

    let (_, exit) = eval(args.clone(), "1 < 1")?;
    assert!(eq(exit, Object::Bool(false))?);

    let (_, exit) = eval(args.clone(), "1 < 2")?;
    assert!(eq(exit, Object::Bool(true))?);

    let (_, exit) = eval(args.clone(), "1 <= 1")?;
    assert!(eq(exit, Object::Bool(true))?);

    let (_, exit) = eval(args.clone(), "1 == 1")?;
    assert!(eq(exit, Object::Bool(true))?);

    let (_, exit) = eval(args.clone(), "1 == 2")?;
    assert!(eq(exit, Object::Bool(false))?);

    let (_, exit) = eval(args.clone(), "1 != 1")?;
    assert!(eq(exit, Object::Bool(false))?);

    let (_, exit) = eval(args.clone(), "1 != 2")?;
    assert!(eq(exit, Object::Bool(true))?);

    Ok(())
}

#[test]
fn condition() -> Result<(), String> {
    let args = Object::List([].into());

    let (_, exit) = eval(args.clone(), "if true { 1 } else { 0 }")?;
    assert!(eq(exit, Object::Int(1))?);

    let (_, exit) = eval(
        args.clone(),
        "x = false; if true { x = 1; } else { return 0; } x",
    )?;
    assert!(eq(exit, Object::Int(1))?);

    let (_, exit) = eval(
        args.clone(),
        "x = false; if true { return 1; } else { x = 0; } x",
    )?;
    assert!(eq(exit, Object::Int(1))?);

    let (_, exit) = eval(args.clone(), "if true { return 1; } else { return 0; }")?;
    assert!(eq(exit, Object::Int(1))?);

    let (_, exit) = eval(
        args.clone(),
        "while true { if true { return 1; } else { return 0; } } 0",
    )?;
    assert!(eq(exit, Object::Int(1))?);

    let (_, exit) = eval(
        Object::Bool(true),
        "while args { if true { break; } else { break; } } 1",
    )?;
    assert!(eq(exit, Object::Int(1))?);

    let (_, exit) = eval(
        args.clone(),
        "if true { if true { if true { return 1; } } } args",
    )?;
    assert!(eq(exit, Object::Int(1))?);

    let (_, exit) = eval(
        args.clone(),
        "if true { if true { if true { return args; } } } 0",
    )?;
    assert!(eq(exit, args.clone())?);

    let (_, exit) = eval(
        args.clone(),
        "if true { if true { if false { return args; } } } 1",
    )?;
    assert!(eq(exit, Object::Int(1))?);

    Ok(())
}

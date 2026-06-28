mod utils;

use crate::utils::exec;
use felys::Object;

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
    exec(args.clone(), "", "0 / 1", Object::Int(0), "")?;
    exec(args.clone(), "", "1 / 1", Object::Int(1), "")?;
    exec(args.clone(), "", "10 / 3", Object::Int(3), "")?;
    exec(args.clone(), "", "0 % 1", Object::Int(0), "")?;
    exec(args.clone(), "", "1 % 1", Object::Int(0), "")?;
    exec(args.clone(), "", "10 % 3", Object::Int(1), "")?;
    exec(args.clone(), "", "-1 % 2", Object::Int(-1), "")?;
    exec(args.clone(), "", "0 * 0", Object::Int(0), "")?;
    exec(args.clone(), "", "0 + 0", Object::Int(0), "")?;
    exec(args.clone(), "", "0 - 0", Object::Int(0), "")?;
    exec(args.clone(), "", "0.0 / 1.0", Object::Float(0.0), "")?;
    exec(args.clone(), "", "1 + 2 * 3", Object::Int(7), "")?;
    exec(args.clone(), "", "(1 + 2) * 3", Object::Int(9), "")?;
    exec(args.clone(), "", "2 * 3 + 4 * 5", Object::Int(26), "")?;
    exec(args.clone(), "", "10 - 5 - 2", Object::Int(3), "")?;
    exec(args.clone(), "", "10 - (5 - 2)", Object::Int(7), "")?;
    exec(args.clone(), "", "2 * 3 / 2", Object::Int(3), "")?;
    exec(args.clone(), "", "10 / 2 / 2", Object::Int(2), "")?;
    exec(args.clone(), "", "-1 + 2", Object::Int(1), "")?;
    exec(args.clone(), "", "-(1 + 2)", Object::Int(-3), "")?;
    exec(args.clone(), "", "-2 * 3", Object::Int(-6), "")?;
    exec(args.clone(), "", "1 + 2 + 3 + 4 + 5", Object::Int(15), "")?;
    exec(
        args.clone(),
        "",
        "1.0 + 2.0 + 3.0 + 4.0 + 5.0",
        Object::Float(15.0),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "1 + 2 * 3 - 4 / 2 + 5",
        Object::Int(10),
        "",
    )?;
    exec(args.clone(), "", "x = 5; x += 3; x", Object::Int(8), "")?;
    exec(args.clone(), "", "x = 10; x -= 4; x", Object::Int(6), "")?;
    exec(args.clone(), "", "x = 3; x *= 4; x", Object::Int(12), "")?;
    exec(args.clone(), "", "x = 20; x /= 4; x", Object::Int(5), "")?;
    exec(args.clone(), "", "x = 17; x %= 5; x", Object::Int(2), "")?;
    exec(
        args.clone(),
        "",
        "x = 2.0; x += 3.0; x",
        Object::Float(5.0),
        "",
    )?;
    exec(args.clone(), "", "x = 5; x += 0; x", Object::Int(5), "")?;
    exec(args.clone(), "", "x = 5; x *= 0; x", Object::Int(0), "")?;

    Ok(())
}

#[test]
fn strings() -> Result<(), String> {
    let args = Object::List([].into());

    exec(args.clone(), "", "\"\"", Object::Str("".into()), "")?;
    exec(args.clone(), "", "\" \"", Object::Str(" ".into()), "")?;
    exec(args.clone(), "", "\"\\n\"", Object::Str("\n".into()), "")?;
    exec(args.clone(), "", "\"\\t\"", Object::Str("\t".into()), "")?;
    exec(args.clone(), "", "\"\\r\"", Object::Str("\r".into()), "")?;
    exec(args.clone(), "", "\"\\\\\"", Object::Str("\\".into()), "")?;
    exec(args.clone(), "", "\"\\\"\"", Object::Str("\"".into()), "")?;
    exec(args.clone(), "", "\"\\u{41}\"", Object::Str("A".into()), "")?;
    exec(
        args.clone(),
        "",
        "\"\\u{1F600}\"",
        Object::Str("😀".into()),
        "",
    )?;
    exec(args.clone(), "", "\"\\u{0}\"", Object::Str("\0".into()), "")?;
    exec(
        args.clone(),
        "",
        "\"a\\nb\\tc\\rd\\\"\"",
        Object::Str("a\nb\tc\rd\"".into()),
        "",
    )?;

    Ok(())
}

#[test]
fn collections() -> Result<(), String> {
    let args = Object::List([].into());

    exec(args.clone(), "", "[]", Object::List([].into()), "")?;
    exec(
        args.clone(),
        "",
        "[1]",
        Object::List([Object::Int(1)].into()),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "[[[]]]",
        Object::List([Object::List([Object::List([].into())].into())].into()),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "(1, 2, 3)",
        Object::Tuple([Object::Int(1), Object::Int(2), Object::Int(3)].into()),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "((1, 2), (3, 4))",
        Object::Tuple(
            [
                Object::Tuple([Object::Int(1), Object::Int(2)].into()),
                Object::Tuple([Object::Int(3), Object::Int(4)].into()),
            ]
            .into(),
        ),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 0; for (a, b) in [(1, 2), (3, 4)] { x = x + a + b; } x",
        Object::Int(10),
        "",
    )?;

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
    exec(
        args.clone(),
        "",
        "true or true or false",
        Object::Bool(true),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "false or false or false",
        Object::Bool(false),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "true and true and true",
        Object::Bool(true),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "true and true and false",
        Object::Bool(false),
        "",
    )?;
    exec(args.clone(), "", "not not true", Object::Bool(true), "")?;
    exec(
        args.clone(),
        "",
        "not not not true",
        Object::Bool(false),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "(true or false) and (true or false)",
        Object::Bool(true),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "(true and false) or (true and true)",
        Object::Bool(true),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "true and false or true",
        Object::Bool(true),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "true and (false or true)",
        Object::Bool(true),
        "",
    )?;
    exec(args.clone(), "", "1 < 2 and 3 > 2", Object::Bool(true), "")?;
    exec(args.clone(), "", "1 + 2 < 4", Object::Bool(true), "")?;
    exec(
        args.clone(),
        "",
        "not true and false",
        Object::Bool(false),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "not (true and false)",
        Object::Bool(true),
        "",
    )?;

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
    exec(args.clone(), "", "0 > 0", Object::Bool(false), "")?;
    exec(args.clone(), "", "0 >= 0", Object::Bool(true), "")?;
    exec(args.clone(), "", "0 < 0", Object::Bool(false), "")?;
    exec(args.clone(), "", "0 <= 0", Object::Bool(true), "")?;
    exec(args.clone(), "", "-1 < 0", Object::Bool(true), "")?;
    exec(args.clone(), "", "-1 > -2", Object::Bool(true), "")?;
    exec(args.clone(), "", "0.0 == 0.0", Object::Bool(true), "")?;
    exec(args.clone(), "", "0.0 != 1.0", Object::Bool(true), "")?;
    exec(args.clone(), "", "0.0 < 1.0", Object::Bool(true), "")?;
    exec(args.clone(), "", "1.0 > 0.0", Object::Bool(true), "")?;
    exec(args.clone(), "", "0.0 <= 0.0", Object::Bool(true), "")?;
    exec(args.clone(), "", "0.0 >= 0.0", Object::Bool(true), "")?;
    exec(args.clone(), "", "-0.0 == 0.0", Object::Bool(true), "")?;
    exec(args.clone(), "", "not (1 == 2)", Object::Bool(true), "")?;

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
    exec(
        args.clone(),
        "",
        "if false { 1 } else { 2 }",
        Object::Int(2),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "if 0 < 1 { if 1 < 2 { 3 } else { 4 } } else { 5 }",
        Object::Int(3),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "if true { if false { 1 } else { 2 } } else { 3 }",
        Object::Int(2),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 0; if true { x = 1; if true { x = 2; } } x",
        Object::Int(2),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "if true and false { 1 } else { 2 }",
        Object::Int(2),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "if true or false { 1 } else { 2 }",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "if 1 < 2 and 2 < 3 { 42 } else { 0 }",
        Object::Int(42),
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
    exec(
        args.clone(),
        "",
        "x = 0; for i in [0, 1, 2, 3] { x += i; } x",
        Object::Int(6),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 0; for (a, b) in [(0, 1), (2, 3)] { x += a + b; } x",
        Object::Int(6),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 0; while false { x = 1; } x",
        Object::Int(0),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 0; for i in [] { x = 1; } x",
        Object::Int(0),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 0; loop { if x == 0 { break; } } x",
        Object::Int(0),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 0; for i in [1, 2, 3] { x += i; if i == 2 { break; } } x",
        Object::Int(3),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 0; for i in [1, 2, 3] { if i == 2 { continue; } x += i; } x",
        Object::Int(4),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 0; loop { x += 1; if x == 5 { continue; } if x >= 10 { break; } } x",
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
    exec(
        args.clone(),
        "fn zero() { return 0; } fn one() { return 1; }",
        "zero()",
        Object::Int(0),
        "",
    )?;
    exec(
        args.clone(),
        "fn id(x) { return x; }",
        "id(42)",
        Object::Int(42),
        "",
    )?;
    exec(
        args.clone(),
        "fn const(x, y) { return x; }",
        "const(1, 2)",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "fn fact(n) { if n <= 1 { return 1; } else { return n * fact(n - 1); } }",
        "fact(5)",
        Object::Int(120),
        "",
    )?;
    exec(
        args.clone(),
        "fn foo() { loop { break 42; } }",
        "foo()",
        Object::Int(42),
        "",
    )?;
    exec(
        args.clone(),
        "fn returns_loop() { loop { break loop { break 99; } } }",
        "returns_loop()",
        Object::Int(99),
        "",
    )?;

    Ok(())
}

#[test]
fn variables() -> Result<(), String> {
    let args = Object::List([].into());

    exec(args.clone(), "", "x = 1; x = x + 1; x", Object::Int(2), "")?;
    exec(
        args.clone(),
        "",
        "x = 1; y = x; x = 2; y",
        Object::Int(1),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 1; if true { x = 2; } else { x = 3; } x",
        Object::Int(2),
        "",
    )?;
    exec(
        args.clone(),
        "",
        "x = 1; loop { if x == 1 { x = 2; break; } } x",
        Object::Int(2),
        "",
    )?;

    Ok(())
}

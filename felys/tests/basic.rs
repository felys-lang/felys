use crate::utils::eval;

mod utils;

#[test]
fn object() -> Result<(), String> {
    let (_, exit) = eval("42")?;
    assert_eq!(exit, "42");

    let (_, exit) = eval("3.14")?;
    assert_eq!(exit, "3.14");

    let (_, exit) = eval("true")?;
    assert_eq!(exit, "true");

    let (_, exit) = eval("\"你好，世界！\"")?;
    assert_eq!(exit, "\"你好，世界！\"");

    let (_, exit) = eval("\"hello, world!\"")?;
    assert_eq!(exit, "\"hello, world!\"");

    let (_, exit) = eval("[0, [0, 0]]")?;
    assert_eq!(exit, "[0, [0, 0]]");

    let (_, exit) = eval("(1, (1, 1))")?;
    assert_eq!(exit, "(1, (1, 1))");

    Ok(())
}

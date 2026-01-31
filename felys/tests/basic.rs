use crate::utils::{eq, eval};
use felys::Object;

mod utils;

#[test]
fn object() -> Result<(), String> {
    let (_, exit) = eval("42")?;
    assert!(eq(exit, Object::Int(42))?);

    let (_, exit) = eval("9.8")?;
    assert!(eq(exit, Object::Float(9.8))?);

    let (_, exit) = eval("true")?;
    assert!(eq(exit, Object::Bool(true))?);

    let (_, exit) = eval("\"你好，世界！\"")?;
    assert!(eq(exit, Object::Str("你好，世界！".into()))?);

    let (_, exit) = eval("\"hello, world!\"")?;
    assert!(eq(exit, Object::Str("hello, world!".into()))?);

    let (_, exit) = eval("[0, [0, 0]]")?;
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

    let (_, exit) = eval("(1, (1, 1))")?;
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

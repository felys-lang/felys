use crate::utils::{eq, eval};
use felys::Object;

mod utils;

#[test]
fn io() -> Result<(), String> {
    let args = Object::List([].into());

    let (stdout, exit) = eval(args.clone(), "std::io::print(42)")?;
    assert_eq!(stdout, "42\n");
    assert!(eq(exit, Object::Int(1))?);

    Ok(())
}

#[test]
fn pink() -> Result<(), String> {
    let args = Object::List([].into());

    let (_, exit) = eval(args.clone(), "std::pink::cyrene()")?;
    assert!(eq(exit, Object::Str("往昔的涟漪♪".into()))?);

    let (_, exit) = eval(args.clone(), "std::pink::elysia()")?;
    assert!(eq(exit, Object::Str("粉色妖精小姐♪".into()))?);

    let (_, exit) = eval(args.clone(), "std::pink::felysneko()")?;
    assert!(eq(exit, Object::Str("银河猫猫侠♪".into()))?);

    Ok(())
}

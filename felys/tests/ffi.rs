use crate::utils::eval;
use felys::Object;

mod utils;

#[test]
fn io() -> Result<(), String> {
    let args = Object::List([].into());

    eval(args.clone(), "std::io::print(42)", Object::Int(1), "42\n")?;

    Ok(())
}

#[test]
fn pink() -> Result<(), String> {
    let args = Object::List([].into());

    eval(
        args.clone(),
        "std::pink::cyrene()",
        Object::Str("往昔的涟漪♪".into()),
        "",
    )?;
    eval(
        args.clone(),
        "std::pink::elysia()",
        Object::Str("粉色妖精小姐♪".into()),
        "",
    )?;
    eval(
        args.clone(),
        "std::pink::felysneko()",
        Object::Str("银河猫猫侠♪".into()),
        "",
    )?;

    Ok(())
}

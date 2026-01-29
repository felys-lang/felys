use crate::utils::eval;

mod utils;

#[test]
fn io() -> Result<(), String> {
    let (stdout, exit) = eval("std::io::print(42)")?;
    assert_eq!(stdout, "42\n");
    assert_eq!(exit, "1");
    Ok(())
}

#[test]
fn pink() -> Result<(), String> {
    let (_, exit) = eval("std::pink::cyrene()")?;
    assert_eq!(exit, "\"往昔的涟漪♪\"");

    let (_, exit) = eval("std::pink::elysia()")?;
    assert_eq!(exit, "\"粉色妖精小姐♪\"");

    let (_, exit) = eval("std::pink::felysneko()")?;
    assert_eq!(exit, "\"银河猫猫侠♪\"");

    Ok(())
}

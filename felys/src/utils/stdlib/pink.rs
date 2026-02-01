use crate::utils::stdlib::utils::Signature;
use crate::Object;
pub const LIB: [(&str, &str, Signature); 3] = [
    ("pink", "elysia", elysia),
    ("pink", "cyrene", cyrene),
    ("pink", "felysneko", felysneko),
];

fn elysia(_: Vec<Object>, _: &mut String) -> Object {
    Object::Str("粉色妖精小姐♪".into())
}

fn cyrene(_: Vec<Object>, _: &mut String) -> Object {
    Object::Str("往昔的涟漪♪".into())
}

fn felysneko(_: Vec<Object>, _: &mut String) -> Object {
    Object::Str("银河猫猫侠♪".into())
}

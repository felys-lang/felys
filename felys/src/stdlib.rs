use crate::Object;

pub type Stdlib = [(&'static str, &'static str, Signature); 4];

pub type Signature = fn(Vec<Object>, &mut String) -> Object;

pub const STDLIB: Stdlib = [
    ("io", "print", PRINT),
    ("pink", "cyrene", CYRENE),
    ("pink", "elysia", ELYSIA),
    ("pink", "felysneko", FELYSNEKO),
];

const PRINT: Signature = |args, stdout| {
    let mut iter = args.iter();
    if let Some(arg) = iter.next() {
        stdout.push_str(&arg.to_string());
    }
    for arg in iter {
        stdout.push(' ');
        stdout.push_str(&arg.to_string());
    }
    stdout.push('\n');
    Object::Int(args.len() as i32)
};

const CYRENE: Signature = |_, _| Object::Str("往昔的涟漪♪".into());

const ELYSIA: Signature = |_, _| Object::Str("粉色妖精小姐♪".into());

const FELYSNEKO: Signature = |_, _| Object::Str("银河猫猫侠♪".into());

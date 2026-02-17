use crate::Object;
use crate::utils::stdlib::nn::operator::Node;

pub type Stdlib = [(&'static str, &'static str, Signature); 5];

pub type Signature = fn(Vec<Object>, &mut String) -> Result<Object, String>;

pub const STDLIB: Stdlib = [
    ("io", "print", PRINT),
    ("pink", "cyrene", CYRENE),
    ("pink", "elysia", ELYSIA),
    ("pink", "felysneko", FELYSNEKO),
    ("nn", "tensor", TENSOR),
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
    Ok(Object::Int(args.len() as i32))
};

const CYRENE: Signature = |_, _| Ok(Object::Str("往昔的涟漪♪".into()));

const ELYSIA: Signature = |_, _| Ok(Object::Str("粉色妖精小姐♪".into()));

const FELYSNEKO: Signature = |_, _| Ok(Object::Str("银河猫猫侠♪".into()));

const TENSOR: Signature = |mut args, _| {
    let object = args.pop().ok_or("expected one argument")?;
    if !args.is_empty() {
        return Err("expected one argument".to_string());
    }
    let node = Node::try_from(object)?;
    Ok(Object::Node(node.into()))
};

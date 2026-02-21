use crate::Object;
use crate::utils::stdlib::nn::operator::Node;

pub type Stdlib = [(&'static str, &'static str, Signature); 9];

pub type Signature = fn(Vec<Object>, &mut String) -> Result<Object, String>;

fn extract<const S: usize>(args: Vec<Object>) -> Result<[Object; S], String> {
    args.try_into()
        .map_err(|_| "invalid number of args".to_string())
}

pub const STDLIB: Stdlib = [
    ("io", "print", PRINT),
    ("pink", "cyrene", CYRENE),
    ("pink", "elysia", ELYSIA),
    ("pink", "felysneko", FELYSNEKO),
    ("nn", "tensor", TENSOR),
    ("nn", "relu", RELU),
    ("nn", "ln", LN),
    ("nn", "exp", EXP),
    ("nn", "sum", SUM),
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

const TENSOR: Signature = |args, _| {
    let [object] = extract(args)?;
    let node = Node::try_from(object)?;
    Ok(Object::Node(node.into()))
};

const RELU: Signature = |args, _| {
    let [object] = extract(args)?;
    let node = Node::relu(object.node()?)?;
    Ok(Object::Node(node))
};

const LN: Signature = |args, _| {
    let [object] = extract(args)?;
    let node = Node::ln(object.node()?)?;
    Ok(Object::Node(node))
};

const EXP: Signature = |args, _| {
    let [object] = extract(args)?;
    let node = Node::exp(object.node()?)?;
    Ok(Object::Node(node))
};

const SUM: Signature = |args, _| {
    let [object, axes] = extract(args)?;
    let mut indices = Vec::new();
    for x in axes.list()?.iter() {
        let int = x
            .int()?
            .try_into()
            .map_err(|_| "invalid axis".to_string())?;
        indices.push(int);
    }
    let node = Node::sum(object.node()?, &indices)?;
    Ok(Object::Node(node))
};

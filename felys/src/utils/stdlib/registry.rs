use crate::Object;
use crate::utils::stdlib::nn::operator::Node;
use std::collections::HashMap;
use std::rc::Rc;

pub type Stdlib<'a> = &'a [(&'static str, &'static str, Signature)];

pub type Signature = fn(Vec<Object>, &mut String) -> Result<Object, String>;

fn extract<const S: usize>(args: Vec<Object>) -> Result<[Object; S], String> {
    args.try_into()
        .map_err(|_| "invalid number of args".to_string())
}

pub const STDLIB: Stdlib = &[
    ("io", "print", PRINT),
    ("pink", "cyrene", CYRENE),
    ("pink", "elysia", ELYSIA),
    ("pink", "felysneko", FELYSNEKO),
    ("utils", "range", RANGE),
    ("nn", "tensor", TENSOR),
    ("nn", "relu", RELU),
    ("nn", "ln", LN),
    ("nn", "exp", EXP),
    ("nn", "sum", SUM),
    ("nn", "mean", MEAN),
    ("nn", "init", INIT),
    ("nn", "attach", ATTACH),
    ("nn", "backward", BACKWARD),
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

const RANGE: Signature = |args, _| {
    let [start, end] = extract(args)?;
    let range = (start.int()?..end.int()?).map(Object::Int).collect();
    Ok(Object::List(range))
};

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
    let [object, axes, keepdim] = extract(args)?;
    let mut indices = Vec::new();
    for x in axes.list()?.iter() {
        let int = x
            .int()?
            .try_into()
            .map_err(|_| "invalid axis".to_string())?;
        indices.push(int);
    }
    indices.dedup();
    let node = Node::sum(object.node()?, &indices, keepdim.bool()?)?;
    Ok(Object::Node(node))
};

const MEAN: Signature = |args, _| {
    let [object, axes, keepdim] = extract(args)?;
    let mut indices = Vec::new();
    for x in axes.list()?.iter() {
        let int = x
            .int()?
            .try_into()
            .map_err(|_| "invalid axis".to_string())?;
        indices.push(int);
    }
    indices.dedup();
    let node = Node::mean(object.node()?, &indices, keepdim.bool()?)?;
    Ok(Object::Node(node))
};

const INIT: Signature = |args, _| {
    fn indexer(i: &mut i32, object: &Object) -> Result<Object, String> {
        match object {
            Object::List(_) => {
                let obj = Object::Int(*i);
                *i = i.checked_add(1).ok_or("integer overflow")?;
                Ok(obj)
            }
            Object::Group(name, subtree) => {
                let body = subtree
                    .iter()
                    .map(|x| indexer(i, x))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Object::Group(*name, body.into()))
            }
            _ => Err("invalid nn module".to_string()),
        }
    }

    fn initializer(object: &Object) -> Result<Object, String> {
        match object {
            Object::List(list) => {
                let mut shape = Vec::with_capacity(list.len());
                for x in list.iter() {
                    let int = x
                        .int()?
                        .try_into()
                        .map_err(|_| "invalid shape".to_string())?;
                    shape.push(int);
                }
                Ok(Object::Node(Node::new(shape.into()).into()))
            }
            Object::Group(name, subtree) => {
                let body = subtree
                    .iter()
                    .map(initializer)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Object::Group(*name, body.into()))
            }
            _ => Err("invalid nn module".to_string()),
        }
    }

    let [object] = extract(args)?;
    Ok(Object::Tuple(
        [indexer(&mut 0, &object)?, initializer(&object)?].into(),
    ))
};

const ATTACH: Signature = |args, _| {
    fn attacher(lhs: &Object, rhs: &Object) -> Result<Object, String> {
        match (lhs, rhs) {
            (Object::Int(lhs), Object::Node(rhs)) => Ok(Object::Node(rhs.attach(*lhs)?.into())),
            (Object::Group(ln, lst), Object::Group(rn, rst)) if ln == rn => {
                let body = lst
                    .iter()
                    .zip(rst.iter())
                    .map(|(lhs, rhs)| attacher(lhs, rhs))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Object::Group(*ln, body.into()))
            }
            _ => Err("cannot attach".to_string()),
        }
    }

    let [lhs, rhs] = extract(args)?;
    attacher(&lhs, &rhs)
};

const BACKWARD: Signature = |args, _| {
    fn backward(lhs: &Object, gradient: &HashMap<i32, Rc<Node>>) -> Result<Object, String> {
        match lhs {
            Object::Group(name, subtree) => {
                let body = subtree
                    .iter()
                    .map(|x| backward(x, gradient))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Object::Group(*name, body.into()))
            }
            Object::Int(x) => {
                let grad = gradient.get(x).cloned().unwrap_or_default();
                Ok(Object::Node(grad))
            }
            _ => Err("invalid nn module".to_string()),
        }
    }
    let [lhs, rhs] = extract(args)?;
    let gradient = rhs.node()?.backward()?;
    backward(&lhs, &gradient)
};

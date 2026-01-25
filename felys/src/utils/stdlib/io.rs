use crate::elysia::Elysia;
use crate::utils::stdlib::utils::Signature;
use crate::Object;

pub const LIB: [(&str, &str, Signature); 1] = [("io", "print", print)];

fn print(args: Vec<Object>, elysia: &Elysia, cs: &mut String) -> Object {
    let mut iter = args.iter();
    if let Some(arg) = iter.next() {
        arg.recover(cs, 0, &elysia.groups).unwrap();
    }
    for arg in iter {
        cs.push(' ');
        arg.recover(cs, 0, &elysia.groups).unwrap();
    }
    cs.push('\n');
    Object::Int(args.len() as isize)
}

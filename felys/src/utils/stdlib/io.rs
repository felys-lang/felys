use crate::elysia::Elysia;
use crate::utils::stdlib::utils::Signature;
use crate::Object;

pub const LIB: [(&str, &str, Signature); 1] = [("io", "print", print)];

fn print(args: Vec<Object>, elysia: &Elysia) -> Object {
    let mut buf = String::new();
    let mut iter = args.into_iter();
    if let Some(arg) = iter.next() {
        arg.recover(&mut buf, 0, &elysia.groups).unwrap();
    }
    for arg in iter {
        buf.push(' ');
        arg.recover(&mut buf, 0, &elysia.groups).unwrap();
    }
    println!("{buf}");
    Object::Void
}

use crate::utils::stdlib::utils::Signature;
use crate::Object;

pub const LIB: [(&str, &str, Signature); 1] = [("io", "print", print)];

fn print(args: Vec<Object>, so: &mut String) -> Object {
    let mut iter = args.iter();
    if let Some(arg) = iter.next() {
        so.push_str(&arg.to_string());
    }
    for arg in iter {
        so.push(' ');
        so.push_str(&arg.to_string());
    }
    so.push('\n');
    Object::Int(args.len() as isize)
}

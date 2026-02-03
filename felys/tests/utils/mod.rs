use felys::{BinOp, Elysia, Object, PhiLia093};

pub fn exec(
    args: Object,
    defs: &str,
    body: &str,
    expect: Object,
    stdout: &str,
) -> Result<(), String> {
    let wrapped = format!("{defs} fn main(args) {{ {body} }}");
    for o in [0, 1, 2, usize::MAX] {
        for elysia in compile(wrapped.as_str(), o)? {
            let mut out = String::new();
            let obj = elysia.exec(args.clone(), &mut out)?;

            if obj.clone().binary(BinOp::Ne, expect.clone())?.bool()? {
                return Err(format!("Expected {}, got {}", expect, obj));
            } else if out != stdout {
                return Err(format!("Expected {}, got {}", stdout, out));
            }
        }
    }

    Ok(())
}

fn compile(code: &str, o: usize) -> Result<[Elysia; 2], String> {
    let elysia = PhiLia093::from(code.to_string())
        .parse()?
        .cfg()?
        .optimize(o)?
        .codegen();

    let mut binary = Vec::with_capacity(256);
    elysia.dump(&mut binary).unwrap();
    let loaded = Elysia::load(&mut binary.as_slice()).unwrap();

    Ok([elysia, loaded])
}

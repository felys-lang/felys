use felys::{BinOp, Elysia, Object, PhiLia093};

pub fn eval(
    args: Object,
    body: &'static str,
    expect: Object,
    stdout: &'static str,
) -> Result<(), String> {
    let wrapped = format!("fn main(args) {{ {body} }}");

    for o in [0, 1, 2, usize::MAX] {
        for elysia in compile(wrapped.clone(), o)? {
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

fn compile(code: String, o: usize) -> Result<[Elysia; 2], String> {
    let elysia = PhiLia093::from(code).parse()?.cfg()?.optimize(o)?.codegen();

    let mut binary = Vec::with_capacity(256);
    elysia.dump(&mut binary).unwrap();
    let loaded = Elysia::load(&mut binary.as_slice()).unwrap();

    Ok([elysia, loaded])
}

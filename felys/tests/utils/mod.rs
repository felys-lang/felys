use felys::{BinOp, Object, PhiLia093, III};

pub fn exec(
    args: Object,
    defs: &str,
    body: &str,
    expect: Object,
    stdout: &str,
) -> Result<(), String> {
    let wrapped = format!("{defs} fn main(args) {{ {body} }}");
    for o in [0, 1, 2, usize::MAX] {
        for iii in compile(wrapped.as_str(), o)? {
            let mut out = String::new();
            let obj = iii.exec(args.clone(), &mut out)?;

            if obj.clone().binary(BinOp::Ne, expect.clone())?.bool()? {
                return Err(format!("Expected {}, got {}", expect, obj));
            } else if out != stdout {
                return Err(format!("Expected {}, got {}", stdout, out));
            }
        }
    }

    Ok(())
}

fn compile(code: &str, _: usize) -> Result<[III; 2], String> {
    let iii = PhiLia093::from(code.to_string())
        .parse()?
        .desugar()?
        .codegen()?;

    let mut binary = Vec::with_capacity(256);
    iii.dump(&mut binary).unwrap();
    let loaded = III::load(&mut binary.as_slice()).unwrap();

    Ok([iii, loaded])
}

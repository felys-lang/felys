use felys::{BinOp, Object, PhiLia093};

pub fn eval(args: Object, body: &'static str) -> Result<(String, Object), String> {
    let wrapped = format!("fn main(args) {{ {body} }}");

    let (uo, ue) = pipeline(wrapped.clone(), args.clone(), 0)?;
    let (oo, oe) = pipeline(wrapped, args, usize::MAX)?;

    if uo != oo {
        Err("inconsistent stdout".to_string())
    } else if ue.clone().binary(BinOp::Ne, oe)?.bool()? {
        Err("inconsistent exit".to_string())
    } else {
        Ok((uo, ue))
    }
}

pub fn eq(lhs: Object, rhs: Object) -> Result<bool, String> {
    lhs.binary(BinOp::Eq, rhs)?.bool().map_err(String::from)
}

fn pipeline(code: String, args: Object, o: usize) -> Result<(String, Object), String> {
    let mut stdout = String::new();
    let exit = PhiLia093::from(code)
        .parse()?
        .cfg()?
        .optimize(o)?
        .codegen()
        .exec(args, &mut stdout)?;
    Ok((stdout, exit))
}

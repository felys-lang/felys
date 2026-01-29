use felys::{Object, PhiLia093};

pub fn eval(body: &'static str) -> Result<(String, String), String> {
    let wrapped = format!("fn main(args) {{ return {body}; }}");
    let args = Object::List([].into());

    let unoptimized = {
        let mut stdout = String::new();
        let exit = PhiLia093::from(wrapped.clone())
            .parse()?
            .cfg()?
            .optimize(0)?
            .codegen()
            .exec(args.clone(), &mut stdout)?;
        (stdout, exit)
    };

    let optimized = {
        let mut stdout = String::new();
        let exit = PhiLia093::from(wrapped)
            .parse()?
            .cfg()?
            .optimize(42)?
            .codegen()
            .exec(args, &mut stdout)?;
        (stdout, exit)
    };

    if unoptimized != optimized {
        return Err("inconsistent generation".to_string());
    }

    Ok(optimized)
}

use std::collections::HashMap;
use felys::*;

fn main() {
    let code = "print('Hello, World');".to_string();
    
    // your customized built-in values and rust functions
    let mixin = HashMap::from([
        ("print".into(), Object::Rust { func: print })
    ]);
    
    // init the worker with mixin, set to no timeout and English mode
    let mut main = Worker::new(mixin, 0.0, Language::EN);
    if let Err(e) = main.exec(code) {
        println!("{}", e)
    }
}

// customize your own `print` function
fn print(cx: &mut Context) -> Output {
    let out = cx.args.iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out);
    Object::None.into()
}

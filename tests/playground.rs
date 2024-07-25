use std::collections::HashMap;
use felys::Language::ENG;
use felys::Worker;

#[test]
fn playground() {
    let code = "__elysia__;".to_string();
    let mut main = Worker::new(HashMap::new(), 0.0, ENG);
    if let Err(e) = main.exec(code) {
        println!("{}", e)
    }
}

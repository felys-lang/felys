use crate::philia093::PhiLia093;

mod ast;
mod philia093;

const CODE: &str = r#"
group Human(age, gender);

impl Human {
    fn new(age, gender) {
        return Human(age, gender);
    }
}

fn main(args) {
    return Human::new(18, "male");
}
"#;

fn main() {
    let mut philia093 = PhiLia093::from(CODE.to_string());
    let root = philia093.root();
    println!("{:?}", philia093.__snapshot);
    println!("{:?}", root);
}

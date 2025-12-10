use crate::philia093::Packrat;

mod ast;
mod philia093;

const CODE: &'static str = r#"
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
    let mut packrat = Packrat::from(CODE.to_string());
    let root = packrat.root();
    println!("{:?}", packrat.__snapshot);
    println!("{:?}", root);
}

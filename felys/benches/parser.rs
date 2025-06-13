use felys::Packrat;
use std::time::Instant;

const PAYLOADS: [&str; 3] = [
    "1.0 + (0.1 >= 4 != (5 <= 6 or 7)) == 8 - 9 and not 0 + (((11 * 12 / 3 % 14 - 1 * +-3 )));",
    "if true { while true { loop { break 1; continue; }}} else if true { x = 10; } else false;",
    "break (3 + 4 * (sin(2)*2)) / (5 - 2 * (if x > 10 { x } else y)) + 3 - -2 \"hello world\";",
];

#[test]
fn parser() {
    for payload in PAYLOADS {
        let code = payload.repeat(1000);
        let start = Instant::now();
        Packrat::from(code).parse().unwrap();
        println!("{:?}", start.elapsed());
    }
}

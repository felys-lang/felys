use felys::parse;
use std::time::Instant;

const PAYLOADS: [&str; 3] = [
    "1 + (2 >= 4 != (5 <= 6 or 7)) == 8 - 9 and 10 + not (((11 * 12 / 13 % 14 - 15 * -16)));",
    "if true { while true { loop { break; continue; }}} else if true { x = 10; } else false;",
    "break (3 + 4 * (math.sin(2)*2)) / (5 - 2 * (if x > 10 { x } else y)) + 3 * 3 - +-+-+-3;"
];

fn main() {
    for payload in PAYLOADS {
        let code = payload.repeat(1000);
        let start = Instant::now();
        if let Err(e) = parse(code) {
            println!("{}", e)
        };
        println!("{:?}", start.elapsed());
    }
}
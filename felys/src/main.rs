use crate::elysia::Object;
use crate::philia093::PhiLia093;
use std::time::Instant;

mod ast;
mod cyrene;
mod demiurge;
mod elysia;
mod philia093;

const CODE: [&str; 6] = [
    r#"
fn main(args) {
    x = 1;
    x = 2;
    if false {
        x = 3;
    }
    0
}
"#,
    r#"
fn main(args) {
    if true {
        if true {
            if true {
                if false {
                    return args;
                }
            }
        }
    }
    0
}
"#,
    r#"
fn main(args) {
    while true {
        if args {
            break;
        } else {
            break;
        }
    }
    0
}
"#,
    r#"
group Vector3(x, y, z);
group Matrix2x2(r1, r2);
group ComplexState(id, data, active);

impl Vector3 {
    fn new(x, y, z) {
        Vector3(x, y, z)
    }

    fn scale(self, other) {
        return Vector3(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z
        );
    }

    fn add(self, other) {
        return Vector3(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z
        );
    }
}

impl Matrix2x2 {
    fn identity() {
        return Matrix2x2(
            Vector3(1.0, 0.0, 0.0),
            Vector3(0.0, 1.0, 0.0)
        );
    }

    fn apply_dot(self, vec) {
        val = self @ vec;
        return val;
    }
}

fn fib_recursive(n) {
    if n <= 1 {
        return n;
    } else {
        return fib_recursive(n - 1) + fib_recursive(n - 2);
    }
}

fn stress_control_flow(limit) {
    counter = 0;
    result = 0;

    while counter < limit {
        if counter % 2 == 0 {
            // for val in [1, 2, 3, 4, 5] {
            //     if val > 3 {
            //         result += val * 2;
            //     } else {
            //         result -= val;
            //     }
            // }
        } else {
            // 测试 break 和 continue
            loop {
                result += 1;
                if result > 10000 {
                    break;
                }
                continue;
            }
        }

        if not (counter == 0) and (result > 100 or result < -100) and true {
            result += 1;
        }

        counter += 1;
    }
    return result;
}

fn string_torture() {
    s1 = "Hello \"World\" \n \t \\";
    s2 = "\u{1F600} Unicode Test";
    return [s1, s2];
}

fn main(args) {
    start_val = 10.5;

    calc = (100.0 + 20.0 * 3.0 / 4.0) - (50.0 % 3.0) + ((1.0 + 2.0) * (3.0 - 1.0));

    v1 = Vector3::new(1.0, 2.0, 3.0);
    v2 = Vector3::new(4.0, 5.0, 6.0);

    v1.scale(2.0);
    v3 = v1.add(v2);

    calc += 10.0;
    calc -= 5.0;
    calc *= 2.0;
    calc /= 1.0;
    calc %= 100.0;

    (a, b) = (10, 20);

    list_val = [100, 200, 300];
    item = list_val[1];

    control_res = stress_control_flow(50);
    fib_res = fib_recursive(10);

    return [
        calc,
        v3,
        control_res,
        fib_res,
        string_torture(),
        item
    ];
}
"#,
    r#"
fn main(args) {
    x = "你好，世界！";
    if args {
        x = args;
    }
    x
}
"#,
    r#"
fn fib(n) {
    if n <= 1 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main(args) {
    fib(10)
}
"#,
];

fn main() -> Result<(), String> {
    let philia093 = PhiLia093::from(CODE[5].to_string());

    let start = Instant::now();
    let cyrene = philia093.parse()?;
    println!("parse: {:?}", start.elapsed());

    let start = Instant::now();
    let demiurge = cyrene.cfg()?.optimize(0)?;
    println!("optimize: {:?}", start.elapsed());

    let start = Instant::now();
    let elysia = demiurge.codegen();
    println!("codegen: {:?}", start.elapsed());

    let start = Instant::now();
    let exit = elysia.exec(Object::Void)?;
    println!("execution: {:?}", start.elapsed());

    // use std::fs::File;
    // use std::io::BufWriter;
    // let file = File::create("test.bin").unwrap();
    // let mut buf = BufWriter::new(file);
    // let start = Instant::now();
    // elysia.dump(&mut buf).map_err(|_| Fault::Runtime)?;
    // println!("dump: {:?}", start.elapsed());

    println!("{:?}", exit);
    Ok(())
}

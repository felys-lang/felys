use std::time::Instant;
use crate::error::Fault;
use crate::philia093::PhiLia093;

mod ast;
mod cyrene;
mod demiurge;
mod elysia;
mod error;
mod philia093;

// const CODE: &str = r#"
// fn main(args) {
//     x = 1;
//     x = 2;
//     if false {
//         x = 3;
//     }
//     0
// }
// "#;

// const CODE: &str = r#"
// fn main(args) {
//     if true {
//         if false {
//             return args;
//         }
//     }
//     0
// }
// "#;

// const CODE: &str = r#"
// fn main(args) {
//     while true {
//         if args {
//             break;
//         } else {
//             break;
//         }
//     }
//     0
// }
// "#;

const CODE: &str = r#"
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
"#;

fn main() -> Result<(), Fault> {
    let start = Instant::now();
    let philia093 = PhiLia093::from(CODE.to_string());
    let cyrene = philia093.parse()?;
    let demiurge = cyrene.cfg()?.optimize()?;
    println!("{}", start.elapsed().as_micros());
    println!("{:?}", demiurge.main.entry);
    for frag in demiurge.main.fragments {
        println!("{:?}", frag);
    }
    println!("{:?}", demiurge.main.exit);
    Ok(())
}

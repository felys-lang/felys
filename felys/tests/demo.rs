use crate::utils::exec;
use felys::Object;

mod utils;

#[test]
fn quickstart() -> Result<(), String> {
    exec(
        Object::List([].into()),
        r#"
// define a function
fn add(x, y) {
    // function must have a return value
    x + y
}
"#,
        r#"
print = std::io::print;

// if-else can have a return value
one = if true {
    1
} else 2;
print("one:", one);

// break a loop with a return value
two = loop {
    break 2;
};
print("two:", two);

// for-loop is only applicable to list
ten = 0;
for (a, b) in [(1, 2), (3, 4)] {
    ten += a + b;
}
print("ten:", ten);

// logical operators
total = 0;
if one == 1 and two == 2 and ten == 10 and true {
    total = one + two + ten;
} else {
    return "unreachable";    
}
print("thirteen:", total);

// while-loop
while total > 5 {
    if total == 8 {
        total -= 5;
        continue;
    }
    total -= 1;
}
print("three:", total);

// exit object
total + add(one, two)
"#,
        Object::Int(6),
        "one: 1\ntwo: 2\nten: 10\nthirteen: 13\nthree: 3\n",
    )
}

#[test]
fn grouping() -> Result<(), String> {
    exec(
        Object::List([].into()),
        r#"
group Vec3(x, y, z);

impl Vec3 {
    fn new(x, y, z) {
        Vec3(x, y, z)
    }

    fn add(self, other) {
        Vec3(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }

    fn mul(self, other) {
        Vec3(
            self.x * other,
            self.y * other,
            self.z * other,
        )
    }

    fn display(self) {
        std::io::print("<", self.x, self.y, self.z, ">")
    }
}
"#,
        r#"
location = Vec3::new(1, 1, 1).mul(3);
other = Vec3::new(1, -2, -3);
location.add(other).display()
"#,
        Object::Int(5),
        "< 4 1 0 >\n",
    )
}

#[test]
fn fibonacci() -> Result<(), String> {
    exec(
        Object::List([].into()),
        r#"
fn fib(n) {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
"#,
        "fib(10)",
        Object::Int(55),
        "",
    )
}

#[test]
fn hoyoverse() -> Result<(), String> {
    exec(
        Object::List([].into()),
        r#"
fn talk(name, to) {
    msg = if name == "Pardofelis" and to == "Mei" {
        "芽衣姐……我……不想死……"
    } else if name == "Focalors" and to == "Neuvillette" {
        "再见纳维莱特，希望你喜欢这五百年来属于你的戏份。"
    } else if name == "Acheron" {
        "我为逝者哀哭……暮雨，终将落下。"
    } else if name == "Astra" {
        "唱著跳著説著，細心編寫遊歷過程，太動聽～"
    } else {
        "Hello, " + to
    };
    name + ": " + msg
}
"#,
        r#"
people = [
        ("Pardofelis", "Mei"),
        ("Focalors", "Neuvillette"),
        ("Acheron", "IX"),
        ("Astra", "Evelyn"),
        ("John Doe", "Jane Doe"),
    ];

    for (name, to) in people {
        msg = talk(name, to);
        std::io::print(msg);
    }

    std::pink::felysneko()
"#,
        Object::Str("银河猫猫侠♪".into()),
        "Pardofelis: 芽衣姐……我……不想死……\n\
        Focalors: 再见纳维莱特，希望你喜欢这五百年来属于你的戏份。\n\
        Acheron: 我为逝者哀哭……暮雨，终将落下。\n\
        Astra: 唱著跳著説著，細心編寫遊歷過程，太動聽～\n\
        John Doe: Hello, Jane Doe\n",
    )
}

#[test]
fn beloved() -> Result<(), String> {
    exec(
        Object::List([].into()),
        "",
        r#"
elysia = std::pink::elysia();
cyrene = std::pink::cyrene();
std::io::print("beloved", elysia, "and", cyrene);
"jonny.jin@uwaterloo.ca"
"#,
        Object::Str("jonny.jin@uwaterloo.ca".into()),
        "beloved 粉色妖精小姐♪ and 往昔的涟漪♪\n",
    )
}

#[test]
fn playground() -> Result<(), String> {
    exec(Object::List([].into()), "", "0", Object::Int(0), "")
}

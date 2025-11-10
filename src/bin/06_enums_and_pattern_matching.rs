// 06_enums_and_pattern_matching.rs
// 枚举与模式匹配

// 定义枚举
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 带有数据的枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // 使用枚举
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // 模式匹配
    let some_number = Some(5);
    match some_number {
        Some(i) => println!("Matched: {}", i),
        None => println!("No value"),
    }

    // if let语法
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("three");
    }

    // 处理Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // 通配模式
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
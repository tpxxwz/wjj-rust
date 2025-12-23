#![allow(unused, dead_code, unreachable_code, path_statements, unexpected_cfgs)]

// === Derive 宏（来自 strum_macros）===
use strum::{
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumMessage, // derive 宏
    EnumProperty,
    EnumString,
    VariantNames,
};

// === Trait（来自 strum）===
// use strum::EnumMessage as EnumMessageTrait;
use strum::IntoEnumIterator; // trait // trait（避免混淆）

//
// 1. 字符串 <-> 枚举 EnumString
// 2. 枚举 <-> 字符串 Display
//
// #[derive(Display, Debug)]
#[derive(EnumString, Display, Debug)]
enum Mode {
    Fast,
    Slow,
    #[strum(serialize = "Turbo1")] // 自定义字符串
    Turbo,
}

// impl std::str::FromStr for Mode {
//     type Err = strum::ParseError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "Fast" => Ok(Mode::Fast),
//             "Slow" => Ok(Mode::Slow),
//             "Turbo1" => Ok(Mode::Turbo), // 来自 #[strum(serialize = "Turbo1")]
//             _ => Err(strum::ParseError::VariantNotFound),
//         }
//     }
// }

#[test]
fn str_enum() {
    println!("=== 1. 字符串 <-> 枚举 ===");
    let m1: Mode = "Fast".parse().unwrap();
    let m2: Mode = "Turbo1".parse().unwrap();
    println!("Parsed: {:?}, {:?}", m1, m2);
    println!("Display: {}", Mode::Slow);
}

//
// 2. 遍历枚举  自动为枚举实现了IntoEnumIterator
//
#[derive(EnumIter, Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

#[test]
fn enum_iter() {
    println!("\n=== 2. 遍历枚举 ===");
    for c in Color::iter() {
        println!("Color variant: {:?}", c);
    }
}

//
// 3. 自动生成 is_xxx()
//
#[derive(EnumIs, Debug)]
enum State {
    Init,
    Running,
    Stopped,
}

#[test]
fn enum_is_xxx() {
    println!("\n=== 3. 自动生成 is_xxx() ===");
    let s = State::Running;
    if s.is_running() {
        println!("State is running");
    }
}

//
// 4. 获取所有 variant 名称
//
#[derive(VariantNames)]
enum Level {
    Low,
    Medium,
    High,
}

#[test]
fn enum_variant() {
    println!("\n=== 4. 获取所有 variant 名称 ===");
    println!("Level variants: {:?}", Level::VARIANTS);
}

//
// 5. 给枚举加属性 多字段属性表
//
#[derive(EnumIs, EnumProperty, Debug)]
enum HttpCode {
    #[strum(props(code = 200, message = "OK"))]
    Ok,

    #[strum(props(code = 404, message = "Not Found"))]
    NotFound,

    #[strum(props(code = 500, message = "Internal Error"))]
    InternalError,
}

// impl EnumProperty for HttpCode {
//     fn get_str(&self, prop: &str) -> Option<&'static str> {
//         match self {
//             HttpCode::Ok => match prop {
//                 "message" => Some("OK"),
//                 _ => None,
//             },
//             HttpCode::NotFound => match prop {
//                 "message" => Some("Not Found"),
//                 _ => None,
//             },
//             HttpCode::InternalError => match prop {
//                 "message" => Some("Internal Error"),
//                 _ => None,
//             },
//         }
//     }
//
//     fn get_int(&self, prop: &str) -> Option<i64> {
//         match self {
//             HttpCode::Ok => match prop {
//                 "code" => Some(200),
//                 _ => None,
//             },
//             HttpCode::NotFound => match prop {
//                 "code" => Some(404),
//                 _ => None,
//             },
//             HttpCode::InternalError => match prop {
//                 "code" => Some(500),
//                 _ => None,
//             },
//         }
//     }
//     fn get_bool(&self, prop: &str) -> Option<bool> {
//         None
//     }
// }
//
// #[test]
// fn enum_property() {
//     println!("\n=== 5. 枚举属性 ===");
//     let code = HttpCode::NotFound;
//     println!(
//         "HttpCode {:?}: code={}, message={}",
//         code,
//         code.get_int("code").unwrap(),
//         code.get_str("message").unwrap(),
//     );
// }

//
// 6. EnumMessage —— 给枚举加 message // 单字段文案
//
#[derive(EnumMessage, Debug)]
enum HttpStatus {
    #[strum(message = "Everything is OK")]
    Ok,

    #[strum(message = "Resource not found")]
    NotFound,

    #[strum(message = "Internal server error")]
    InternalError,
}

#[test]
fn enum_message_demo() {
    println!("=== EnumMessage Demo ===");

    let s = HttpStatus::NotFound;

    // get_message() 返回 Option<&'static str>
    println!("Status {:?} => message: {}", s, s.get_message().unwrap());
}

//
// 7. EnumCount —— 自动生成 COUNT 常量
//
#[derive(EnumCount, Debug)]
enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

#[test]
fn enum_count_demo() {
    println!("\n=== EnumCount Demo ===");

    // 自动生成 Priority::COUNT 常量
    println!("Priority count = {}", Priority::COUNT);
}

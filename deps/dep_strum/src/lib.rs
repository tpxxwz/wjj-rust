#![allow(unused, dead_code, unreachable_code, path_statements, unexpected_cfgs)]

// === Derive 宏（来自 strum_macros）===
use strum::{
    AsRefStr, Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumMessage, EnumProperty,
    EnumString, EnumTryAs, FromRepr, IntoEnumIterator, IntoStaticStr, VariantArray, VariantNames,
};

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
// 3. 遍历枚举  自动为枚举实现了IntoEnumIterator
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
// 4. 自动生成 is_xxx()
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
// 5. 获取所有 variant 名称 纯字符串
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
    let variants: &'static [&'static str] = Level::VARIANTS;
    println!("Level variants: {:?}", variants);
}

// 6. TerrainType 生成一个 不可变借用形成的「切片（slice）」
#[derive(VariantArray, Debug)]
enum TerrainType {
    Desert,
    Forest,
    Ocean,
}

#[test]
fn demo_variant_array() {
    let variants: &[TerrainType] = TerrainType::VARIANTS;
    println!("{:?}", variants);
}

//
// 7. 给枚举加属性 多字段属性表
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
// 8. EnumMessage —— 给枚举加 message // 单字段文案
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
// 9. EnumCount —— 自动生成 COUNT 常量
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

// 10. 整数类型 <-> 枚举 FromRepr
#[derive(FromRepr, Debug)]
#[repr(u8)]
enum PacketType {
    Init = 1,
    Data = 2,
    Close = 3,
}

#[test]
fn demo_from_repr() {
    let p = PacketType::from_repr(2).unwrap();
    println!("{:?}", p);
}

// 11. AsRefStr 会自动为你的 enum 实现 AsRef<str>，让枚举值可以被借用成 &str。
// 完全不会改里面的字符串的
#[derive(AsRefStr, Debug)]
enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

#[test]
fn demo_as_ref_str() {
    let s = ShaderStage::Compute;
    println!("{}", s.as_ref());
}
// 12. IntoStaticStr —— 枚举 → &'static str 的零成本转换
#[derive(IntoStaticStr, Debug)]
enum AudioFormat {
    Mp3,
    Flac,
    Wav,
}

#[test]
fn demo_into_static_str() {
    let f: &'static str = AudioFormat::Flac.into();
    println!("{}", f);
}
// 13. EnumDiscriminants 会为你的枚举生成一个“只包含 discriminant 名称的新枚举”。 判别式
/*
===============================================================
🧠 EnumDiscriminants —— 从“带数据的 enum”中提取“纯判别式 enum”
---------------------------------------------------------------
用途：
- 日志系统：只记录事件类型，不记录 payload
- 监控统计：统计每种事件出现次数
- 序列化：避免把敏感 payload 序列化出去
- 状态机：只关心状态类型，不关心数据
===============================================================
*/

#[derive(EnumDiscriminants, Debug)]
enum Event {
    Click { x: i32, y: i32 },
    Key(char),
    Quit,
}

#[test]
fn demo_enum_discriminants_real() {
    let e1 = Event::Click { x: 10, y: 20 };
    let e2 = Event::Key('A');
    let e3 = Event::Quit;

    // 自动生成的判别式 enum：EventDiscriminants
    let d1: EventDiscriminants = e1.into();
    let d2: EventDiscriminants = e2.into();
    let d3: EventDiscriminants = e3.into();

    // 现在你可以只根据“类型”做逻辑，而不关心 payload
    match d1 {
        EventDiscriminants::Click => println!("clicked"),
        EventDiscriminants::Key => println!("key pressed"),
        EventDiscriminants::Quit => println!("quit"),
    }

    // 如果不贴 EventDiscriminants 下面match Click { x: 1, y: 1 } 这里就得写各种各样的 规则全部判断
    // let d1: Event = e1.into();
    // let d2: Event = e2.into();
    // let d3: Event = e3.into();
    //
    // let e_demo = Event::Click { x: 10, y: 20 };
    // match e_demo {
    //     Event::Click { x, y}  => println!("clicked"),
    //     Event::Key => println!("key pressed"),
    //     Event::Quit => println!("quit"),
    // }

    println!("d1={:?}, d2={:?}, d3={:?}", d1, d2, d3);
}

// 14. EnumTryAs 就是为message生成try_as_的不同方法
// EnumTryAs 的核心语义是：
// “如果是这个变体，把里面的数据借出来”
// fn try_as_key(&self) -> Option<&char>; 想要生成这样的
#[derive(EnumTryAs, Debug)]
enum Message {
    Quit,                       // ❌ 不生成方法（unit variant）
    Move { x: i32, y: i32 },    // ❌ 不生成方法（struct variant） ??? 好像是返回值Option<&Move>; // Move不是struct只是一个enum Message
    Write(String),              // ✔ 生成 try_as_write()
    ChangeColor(i32, i32, i32), // ✔ 生成 try_as_change_color()
}

#[test]
fn demo_enum_try_as() {
    let m1 = Message::Write("Hello".into());
    let m2 = Message::ChangeColor(1, 2, 3);
    let m3 = Message::Quit;

    // 自动生成的方法（来自 EnumTryAs）
    assert_eq!(m1.try_as_write(), Some("Hello".to_string()));
    assert_eq!(m2.try_as_change_color(), Some((1, 2, 3)));

    // 不匹配 → None
    assert_eq!(m3.try_as_write(), None);

    println!("EnumTryAs demo passed!");
}

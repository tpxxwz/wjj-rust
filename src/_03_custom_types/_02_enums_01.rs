// 枚举
// enum 关键字允许创建一个可能是几种不同变体之一的类型。任何作为 struct 有效的变体在 enum 中也是有效的。

// 创建一个 `enum` 来分类网页事件。注意名称和类型信息如何共同指定变体：
// `PageLoad != PageUnload` 且 `KeyPress(char) != Paste(String)`。
// 每个变体都是不同且独立的。
enum WebEvent {
    // `enum` 变体可以类似单元结构体（`unit-like`），
    PageLoad,
    PageUnload,
    // 类似元组结构体，
    KeyPress(char),
    Paste(String),
    // 或类似 C 语言的结构体。
    Click { x: i64, y: i64 },
}

// 一个接受 `WebEvent` 枚举作为参数且不返回任何值的函数。
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("页面已加载"),
        WebEvent::PageUnload => println!("页面已卸载"),
        // 从 `enum` 变体内部解构 `c`。
        WebEvent::KeyPress(c) => println!("按下了'{}'键。", c),
        WebEvent::Paste(s) => println!("粘贴了\"{}\"。", s),
        // 将 `Click` 解构为 `x` 和 `y`。
        WebEvent::Click { x, y } => {
            println!("点击坐标：x={}, y={}。", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从字符串切片创建一个拥有所有权的 `String`。
    let pasted  = WebEvent::Paste("我的文本".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

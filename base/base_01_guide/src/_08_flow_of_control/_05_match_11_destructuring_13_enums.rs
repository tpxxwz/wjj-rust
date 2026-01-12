// 枚举
// enum 的解构方式类似：

// 使用 `allow` 来抑制警告，因为只使用了一个变体。
#[allow(dead_code)]
enum Color {
    // 这 3 个仅通过名称指定。
    Red,
    Blue,
    Green,
    // 这些同样将 `u32` 元组与不同的名称（颜色模型）关联。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[test]
fn main() {
    let color = Color::RGB(122, 17, 40);
    // TODO ^ 尝试为 `color` 使用不同的变体

    println!("这是什么颜色？");
    // 可以使用 `match` 来解构 `enum`。
    match color {
        Color::Red => println!("颜色是红色！"),
        Color::Blue => println!("颜色是蓝色！"),
        Color::Green => println!("颜色是绿色！"),
        Color::RGB(r, g, b) => println!("红：{}，绿：{}，蓝：{}！", r, g, b),
        Color::HSV(h, s, v) => println!("色相：{}，饱和度：{}，明度：{}！", h, s, v),
        Color::HSL(h, s, l) => println!("色相：{}，饱和度：{}，亮度：{}！", h, s, l),
        Color::CMY(c, m, y) => println!("青：{}，品红：{}，黄：{}！", c, m, y),
        Color::CMYK(c, m, y, k) => println!("青：{}，品红：{}，黄：{}，黑（K）：{}！", c, m, y, k),
        // 不需要其他分支，因为所有变体都已检查
    }
}

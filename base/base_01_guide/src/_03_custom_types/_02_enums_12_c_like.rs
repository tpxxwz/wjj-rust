// C 风格用法
// enum 也可以像 C 语言那样使用。

// 这个属性用于隐藏未使用代码的警告。
#![allow(dead_code)]

// 带隐式判别值的枚举（从 0 开始）
enum Number {
    Zero,
    One,
    Two,
}

// 带显式判别值的枚举
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

#[wjj_lib::gen_test]
fn main() {
    // `enum` 可以转换为整数。
    println!("zero 的值是 {}", Number::Zero as i32);
    println!("one 的值是 {}", Number::One as i32);

    println!("玫瑰的颜色是 #{:06x}", Color::Red as u32);
    println!("紫罗兰的颜色是 #{:06x}", Color::Blue as u32);
}


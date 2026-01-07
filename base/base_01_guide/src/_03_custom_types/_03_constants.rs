// 常量
// Rust 有两种常量类型，可以在任何作用域（包括全局作用域）中声明。两者都需要显式类型标注：
//
// const：不可变值（常见用法）。
// static：具有 'static 生命周期的可能可变变量。静态生命周期会被自动推断，无需明确指定。访问或修改可变静态变量是 unsafe 的。

// 全局变量在所有其他作用域之外声明。
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // 在函数中访问常量
    n > THRESHOLD
}

#[test]
fn main() {
    let n = 16;

    // 在主线程中访问常量
    println!("这是 {}", LANGUAGE);
    println!("阈值为 {}", THRESHOLD);
    println!("{} 是 {}", n, if is_big(n) { "大的" } else { "小的" });

    // 错误！不能修改 `const`。
    // THRESHOLD = 5;
    // 修复：^ 注释掉此行
}

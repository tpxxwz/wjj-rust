// From 和 Into 是可互换的
// From 和 Into 被设计为互补的。我们不需要为两个特质都提供实现。
// 如果你为你的类型实现了 From 特质，Into 会在必要时调用它。
// 但请注意，反过来并不成立：为你的类型实现 Into 不会自动为它提供 From 的实现。

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// 定义 `From`
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // 使用 `Into`
    let num: Number = int.into();
    println!("我的数字是 {:?}", num);
}
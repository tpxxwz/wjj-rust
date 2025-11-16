// Into
// Into 特质简单来说就是 From 特质的反向操作。它定义了如何将一个类型转换为另一个类型。
//
// 调用 into() 通常需要我们指定结果类型，因为编译器大多数时候无法确定这一点。

use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

fn main() {
    let int = 5;
    // 尝试移除类型标注
    let num: Number = int.into();
    println!("我的数字是 {:?}", num);
}

crate::gen_test!(main);
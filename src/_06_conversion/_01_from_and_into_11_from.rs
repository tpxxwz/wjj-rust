// From
// From 特质允许一个类型定义如何从另一个类型创建自身，从而提供了一种非常简单的机制来在多种类型之间进行转换。标准库中有许多这个特质的实现，用于原始类型和常见类型的转换。
//
// 例如，我们可以轻松地将 str 转换为 String
fn str2string() {
    let my_str = "hello";
    let my_string = String::from(my_str);
}
// 我们可以为自己的类型定义类似的转换。
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("我的数字是 {:?}", num);
}

crate::gen_test!(main);
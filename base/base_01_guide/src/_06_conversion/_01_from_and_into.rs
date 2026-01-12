// From 和 Into
// From 和 Into 特质本质上是相互关联的，这实际上是其实现的一部分。
// 如果你能将类型 A 从类型 B 转换，那么我们也应该能够将类型 B 转换为类型 A。

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

// 定义 `From`
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[test]
fn main1() {
    let num = Number::from(30);
    println!("我的数字是 {:?}", num);
}

// Into
// Into 特质简单来说就是 From 特质的反向操作。它定义了如何将一个类型转换为另一个类型。
//
// 调用 into() 通常需要我们指定结果类型，因为编译器大多数时候无法确定这一点。

use std::convert::Into;

// impl Into<Number> for i32 { ... } // 因为上面定义了Number from i32 所以自动生成了 i32 from Number
// ** 反过来先定义 i32 from Number 不能自动生成 Number from i32 **
//
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

#[test]
fn main2() {
    let int = 5;
    // 尝试移除类型标注
    let num: Number = int.into();
    println!("我的数字是 {:?}", num);
}

// From 和 Into 是可互换的
// From 和 Into 被设计为互补的。我们不需要为两个特质都提供实现。
// 如果你为你的类型实现了 From 特质，Into 会在必要时调用它。
// ** 但请注意，反过来并不成立：为你的类型实现 Into 不会自动为它提供 From 的实现。
#[test]
fn main3() {
    let int = 5;
    // 使用 `Into`
    let num: Number = int.into();
    println!("我的数字是 {:?}", num);
}

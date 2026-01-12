// 运算符重载
// 在 Rust 中，许多运算符可以通过 trait 进行重载。
// 这意味着某些运算符可以根据输入参数执行不同的任务。
// 之所以可能，是因为运算符实际上是方法调用的语法糖。
// 例如，a + b 中的 + 运算符会调用 add 方法（相当于 a.add(b)）。
// 这个 add 方法是 Add trait 的一部分。
// 因此，任何实现了 Add trait 的类型都可以使用 + 运算符。
//
// 可以在 core::ops 模块中找到用于重载运算符的 trait 列表，如 Add 等。

use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// `std::ops::Add` trait 用于指定 `+` 的功能
// 这里我们实现 `Add<Bar>` - 这个 trait 用于与 `Bar` 类型的右操作数相加
// 下面的代码块实现了操作：Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> 调用了 Foo.add(Bar)");

        FooBar
    }
}

// 通过交换类型顺序，我们实现了非交换加法。
// 这里我们实现 `Add<Foo>` trait —— 用于处理右操作数类型为 `Foo` 的加法。
// 此代码块实现了如下操作：Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> 调用了 Bar.add(Foo)");

        BarFoo
    }
}

#[test]
fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

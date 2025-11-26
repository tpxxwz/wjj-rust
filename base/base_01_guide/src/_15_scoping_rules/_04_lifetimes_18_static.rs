// 静态
// Rust 有几个保留的生命周期名称。其中之一是 'static。你可能在两种情况下遇到它
// 具有 'static 生命周期的引用：
// let s: &'static str = "你好，世界";
//
// 'static 作为 trait 约束的一部分：
// fn generic<T>(x: T) where T: 'static {}
//
// 这两种情况虽然相关但有微妙的区别，这也是学习 Rust 时常见的困惑来源。以下是每种情况的一些例子：

// 引用生命周期
// 作为引用生命周期，'static 表示该引用指向的数据在程序的整个剩余运行期间都有效。它仍然可以被强制转换为更短的生命周期。
//
// 有两种常见的方法可以创建具有 'static 生命周期的变量，它们都存储在二进制文件的只读内存中：
//
// 使用 static 声明创建一个常量。
// 创建一个字符串字面量，其类型为：&'static str。
// 请看下面的例子，展示了这些方法：

// 创建一个具有 `'static` 生命周期的常量。
static NUM: i32 = 18;

// 返回一个指向 `NUM` 的引用，其中 `'static` 生命周期
// 被强制转换为输入参数的生命周期。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

#[wjj_lib::gen_test]
fn main1() {
    {
        // 创建一个字符串字面量并打印它：
        let static_string = "我存储在只读内存中";
        println!("static_string 的值：{}", static_string);

        // 当 `static_string` 离开作用域时，该引用
        // 不能再被使用，但数据仍然保留在二进制文件中。
    }

    {
        // 创建一个整数用于 `coerce_static` 函数：
        let lifetime_num = 9;

        // 将 `NUM` 的生命周期强制转换为与 `lifetime_num` 一致：
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM：{} 仍然可以访问！", NUM);
}

// 'static 引用只需在程序生命周期的剩余部分有效，因此可以在程序执行过程中创建。
// 为了演示这一点，下面的例子使用 Box::leak 动态创建 'static 引用。
// 在这种情况下，它显然不会存在于整个程序生命周期，而只是从泄漏点开始存在。

extern crate rand;
use rand::{Fill, Rng};

fn random_vec() -> &'static [usize; 100] {
    let mut rng = rand::rng();
    let mut boxed = Box::new([0; 100]);
    for x in boxed.iter_mut() {
        *x = rng.random_range(0..usize::MAX);
    }
    Box::leak(boxed)
}

#[wjj_lib::gen_test]
fn main2() {
    let first: &'static [usize; 100] = random_vec();
    let second: &'static [usize; 100] = random_vec();
    assert_ne!(first, second)
}

// 'static 引用只需在程序生命周期的剩余部分有效，因此可以在程序执行过程中创建。
// 为了演示这一点，下面的例子使用 Box::leak 动态创建 'static 引用。
// 在这种情况下，它显然不会存在于整个程序生命周期，而只是从泄漏点开始存在。

#[wjj_lib::gen_test]
fn main3() {
    let first: &'static [usize; 100] = random_vec();
    let second: &'static [usize; 100] = random_vec();
    assert_ne!(first, second)
}

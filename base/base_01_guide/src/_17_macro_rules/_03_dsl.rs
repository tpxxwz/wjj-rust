// 领域特定语言（DSL）Domain Specific Languages (DSLs)
// DSL 是嵌入在 Rust 宏中的一种微型"语言"。
// 它是完全有效的 Rust 代码，因为宏系统会将其展开为普通的 Rust 结构，但它看起来像一种小型语言。
// 这使你能够为某些特定功能定义简洁或直观的语法（在一定范围内）。
//
// 假设我想定义一个简单的计算器 API。我希望提供一个表达式，并将计算结果打印到控制台。

macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // 强制类型为无符号整数
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

#[wjj_lib::gen_test]
fn main() {
    calculate! {
        eval 1 + 2 // 嘿嘿，`eval` 可不是 Rust 的关键字哦！
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
// 输出：
//
// 1 + 2 = 3
// (1 + 2) * (3 / 4) = 0
// 这个例子非常简单，但已经有很多利用宏开发的复杂接口，比如 lazy_static 或 clap。
//
// 另外，注意宏中的两对大括号。外层的大括号是 macro_rules! 语法的一部分，除此之外还可以使用 () 或 []。


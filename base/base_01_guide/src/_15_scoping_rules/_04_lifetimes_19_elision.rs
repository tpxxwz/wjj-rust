// 省略
// 有些生命周期模式非常常见，因此借用检查器允许省略它们以减少代码量并提高可读性。
// 这被称为省略。Rust 中的省略存在的唯一原因是这些模式很常见。
//
// 以下代码展示了一些省略的例子。要更全面地了解省略，请参阅 Rust 程序设计语言中的生命周期省略章节。

// `elided_input` 和 `annotated_input` 本质上具有相同的签名
// 因为 `elided_input` 的生命周期是由编译器推断的：
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

// 同样，`elided_pass` 和 `annotated_pass` 具有相同的签名
// 因为生命周期被隐式地添加到 `elided_pass`：
fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

#[test]
fn main() {
    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}

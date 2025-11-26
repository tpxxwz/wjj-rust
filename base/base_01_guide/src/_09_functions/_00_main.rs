// 函数
// 函数使用 fn 关键字声明。函数参数需要标注类型，就像变量一样。
// 如果函数返回值，则必须在箭头 -> 后指定返回类型。
//
// 函数的最后一个表达式将作为返回值。
// 另外，可以使用 return 语句在函数内部提前返回值，甚至可以在循环或 if 语句内部使用。
//
// 让我们用函数重写 FizzBuzz 吧！

// 与 C/C++ 不同，Rust 中函数定义的顺序没有限制
#[wjj_lib::gen_test]
fn main() {
    // 我们可以在这里使用函数，并在稍后的某处定义它
    fizzbuzz_to(100);
}

// 返回布尔值的函数
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 特殊情况，提前返回
    if rhs == 0 {
        return false;
    }

    // 这是一个表达式，此处不需要 `return` 关键字
    lhs % rhs == 0
}

// "无返回值"的函数实际上返回单元类型 `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// 当函数返回 `()` 时，可以在函数签名中省略返回类型
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}


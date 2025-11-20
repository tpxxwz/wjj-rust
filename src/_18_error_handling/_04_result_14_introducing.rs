// 引入 ?
// 有时我们只想要 unwrap 的简单性，而不希望有 panic 的可能。
// 到目前为止，当我们真正想要的是获取变量值时，unwrap 迫使我们不断地增加嵌套。这正是 ? 运算符的目的。
//
// 当遇到 Err 时，有两种可行的处理方式：
//
// 使用 panic!（我们已经决定尽可能避免这种方式）
// 使用 return（因为 Err 表示无法处理该错误）
// ? 运算符几乎1等同于在遇到 Err 时执行 return 而非 panic 的 unwrap。让我们看看如何简化之前使用组合器的例子：

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n 是 {}", n),
        Err(e) => println!("错误：{}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}

crate::gen_test!(main);
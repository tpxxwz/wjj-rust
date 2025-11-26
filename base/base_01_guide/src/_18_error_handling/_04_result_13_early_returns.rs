// 提前返回
// 在前面的例子中，我们使用组合器显式地处理了错误。处理这种情况分析的另一种方法是使用 match 语句和提前返回的组合。
//
// 也就是说，如果发生错误，我们可以简单地停止执行函数并返回错误。
// 对某些人来说，这种形式的代码可能更容易阅读和编写。考虑使用提前返回重写的前面示例的这个版本：

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number)  => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number)  => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n 是 {}", n),
        Err(e) => println!("错误：{}", e),
    }
}

#[wjj_lib::gen_test]
fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}

// 到目前为止，我们已经学会了使用组合器和提前返回来显式处理错误。
// 虽然我们通常想避免 panic，但显式处理所有错误是很繁琐的。
// 在下一节中，我们将介绍 ? 运算符，用于我们只需要 unwrap 而不可能引发 panic 的情况。


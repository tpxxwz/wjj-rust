// Result 的别名
// 如果我们想多次重用特定的 Result 类型，该怎么办？
// 回想一下，Rust 允许我们创建别名。方便的是，我们可以为特定的 Result 定义一个别名。
//
// 在模块级别，创建别名特别有用。
// 在特定模块中发现的错误通常具有相同的 Err 类型，因此单个别名可以简洁地定义所有相关的 Result。
// 这非常实用，以至于 std 库甚至提供了一个：io::Result！
//
// 这里有一个简单的例子来展示语法：

use std::num::ParseIntError;

// 为错误类型为 `ParseIntError` 的 `Result` 定义一个泛型别名。
type AliasedResult<T> = Result<T, ParseIntError>;

// 使用上面定义的别名来引用我们特定的 `Result` 类型。
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

// 在这里，别名再次让我们节省了一些代码空间。
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n 是 {}", n),
        Err(e) => println!("错误：{}", e),
    }
}

#[test]
fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}

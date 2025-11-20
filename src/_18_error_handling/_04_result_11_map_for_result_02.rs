// 幸运的是，Option 的 map、and_then 以及许多其他组合器也为 Result 实现了。Result 文档中包含了完整的列表。

use std::num::ParseIntError;

// 与 `Option` 类似，我们可以使用诸如 `map()` 之类的组合器。
// 这个函数除此之外与上面的函数相同，其含义为：
// 如果两个值都可以从字符串解析，则相乘；否则传递错误。
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n 是 {}", n),
        Err(e) => println!("错误：{}", e),
    }
}

fn main() {
    // 这仍然给出了一个合理的答案。
    let twenty = multiply("10", "2");
    print(twenty);

    // 以下代码现在提供了一个更有帮助的错误消息。
    let tt = multiply("t", "2");
    print(tt);
}

crate::gen_test!(main);
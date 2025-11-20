// Result 的 map
// 在前面示例的 multiply 函数中使用 panic 并不能产生健壮的代码。
// 通常，我们希望将错误返回给调用者，让它决定如何正确地处理错误。
//
// 首先，我们需要知道我们正在处理的错误类型。
// 要确定 Err 类型，我们可以查看 parse() 方法，它是通过 FromStr trait 为 i32 实现的。
// 因此，Err 类型被指定为 ParseIntError。
//
// 在下面的示例中，直接使用 match 语句会导致整体代码更加繁琐。

use std::num::ParseIntError;

// 重写返回类型后，我们使用模式匹配而不是 `unwrap()`。
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number)  => {
            match second_number_str.parse::<i32>() {
                Ok(second_number)  => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
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
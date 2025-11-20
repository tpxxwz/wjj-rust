// 在 main 函数中使用 Result
// 如果显式指定，Result 类型也可以作为 main 函数的返回类型。通常，main 函数的形式如下：

// fn main() {
//     println!("Hello World!");
// }

// 然而，main 函数也可以返回 Result 类型。如果 main 函数内发生错误，它将返回一个错误代码并打印该错误的调试表示（使用 Debug trait）。以下示例展示了这种情况，并涉及了下一节中讨论的内容。

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}

crate::gen_test!(main);

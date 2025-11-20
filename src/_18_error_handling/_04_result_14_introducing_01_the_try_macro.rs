// try! 宏
// 在 ? 运算符出现之前，相同的功能是通过 try! 宏实现的。现在推荐使用 ? 运算符，但在查看旧代码时，你可能仍会遇到 try!。使用 try! 宏，前面例子中的 multiply 函数会是这样的

// 要使用 Cargo 编译并运行此示例而不出错，请将 `Cargo.toml` 文件中
// `[package]` 部分的 `edition` 字段值更改为 "2015"。

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    // try!宏已被废弃
    // let first_number = try!(first_number_str.parse::<i32>());
    // let second_number = try!(second_number_str.parse::<i32>());

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
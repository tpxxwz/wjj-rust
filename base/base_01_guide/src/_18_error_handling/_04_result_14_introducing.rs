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

fn multiply1(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n 是 {}", n),
        Err(e) => println!("错误：{}", e),
    }
}

#[wjj_lib::gen_test]
fn main1() {
    print(multiply1("10", "2"));
    print(multiply1("t", "2"));
}

// try! 宏
// 在 ? 运算符出现之前，相同的功能是通过 try! 宏实现的。现在推荐使用 ? 运算符，但在查看旧代码时，你可能仍会遇到 try!。使用 try! 宏，前面例子中的 multiply 函数会是这样的

// 要使用 Cargo 编译并运行此示例而不出错，请将 `Cargo.toml` 文件中
// `[package]` 部分的 `edition` 字段值更改为 "2015"。

fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    // try!宏已被废弃
    // let first_number = try!(first_number_str.parse::<i32>());
    // let second_number = try!(second_number_str.parse::<i32>());

    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

#[wjj_lib::gen_test]
fn main2() {
    print(multiply2("10", "2"));
    print(multiply2("t", "2"));
}

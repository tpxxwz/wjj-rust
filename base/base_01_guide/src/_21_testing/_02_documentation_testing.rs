// 文档测试
// Rust 项目的主要文档编写方式是通过在源代码中添加注释。
// 文档注释使用 CommonMark Markdown 规范编写，并支持其中的代码块。
// Rust 注重正确性，因此这些代码块会被编译并用作文档测试。
/// 第一行是函数的简要描述。
///
/// 接下来的几行是详细文档。代码块以三个反引号开始，
/// 并隐含了 `fn main()` 函数和 `extern crate <cratename>` 声明。
/// 假设我们正在测试 `playground` crate：
///
/// ```
/// let result = playground::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 文档注释通常包含"示例"、"异常"和"错误"等部分。
///
/// 下面的函数用于两数相除。
///
/// # 示例
///
/// ```
/// let result = playground::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # 异常
///
/// 当第二个参数为零时，函数会触发异常。
///
/// ```rust,should_panic
/// // 除以零会触发异常
/// playground::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("除以零错误");
    }

    a / b
}

// 运行常规的 cargo test 命令时，文档中的代码块会自动进行测试：
// $ cargo test
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//
// Doc-tests playground
//
// running 3 tests
// test src/lib.rs - add (line 7) ... ok
// test src/lib.rs - div (line 21) ... ok
// test src/lib.rs - div (line 31) ... ok
//
// test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

// 文档测试的动机
// 文档测试的主要目的是提供功能演示的示例，这是最重要的指导原则之一。
// 它允许将文档中的示例作为完整的代码片段使用。但是使用 ? 会导致编译失败，因为 main 函数返回 unit 类型。
// 这时，隐藏文档中的某些源代码行就派上用场了：
// 可以编写 fn try_main() -> Result<(), ErrorType>，将其隐藏，并在隐藏的 main 函数中 unwrap 它。
// 听起来很复杂？这里有一个例子：
/// 在文档测试中使用隐藏的 `try_main` 函数。
///
/// ```
/// # // 以 `#` 开头的行在文档中是隐藏的，但它们仍然可以编译！
/// # fn try_main() -> Result<(), String> { // 这行包装了文档中显示的函数体
/// let res = playground::try_div(10, 2)?;
/// # Ok(()) // 从 try_main 返回
/// # }
/// # fn main() { // 开始会调用 unwrap() 的 main 函数
/// #    try_main().unwrap(); // 调用 try_main 并解包
/// #                         // 这样在出错时测试会触发 panic
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("除以零错误"))
    } else {
        Ok(a / b)
    }
}

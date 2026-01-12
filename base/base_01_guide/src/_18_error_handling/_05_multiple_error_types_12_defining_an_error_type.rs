// 定义错误类型
// 有时，用单一类型的错误来掩盖所有不同的错误可以简化代码。我们将通过自定义错误来演示这一点。
//
// Rust 允许我们定义自己的错误类型。通常，一个"好的"错误类型应该：
//
// 用同一类型表示不同的错误
// 向用户展示友好的错误消息
// 易于与其他类型进行比较
// 好的示例：Err(EmptyVec)
// 不好的示例：Err("请使用至少包含一个元素的向量".to_owned())
// 能够保存错误的相关信息
// 好的示例：Err(BadChar(c, position))
// 不好的示例：Err("此处不能使用 +".to_owned())
// 能够与其他错误很好地组合

use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

// 定义我们的错误类型。这些可以根据我们的错误处理情况进行自定义。
// 现在我们可以编写自己的错误，依赖底层的错误实现，
// 或者在两者之间做一些处理。
#[derive(Debug, Clone)]
struct DoubleError;

// 错误的生成与其显示方式是完全分离的。
// 无需担心显示样式会使复杂的逻辑变得混乱。
//
// 注意，我们没有存储关于错误的任何额外信息。这意味着如果不修改类型
// 来携带相关信息，我们就无法指出具体是哪个字符串解析失败了。
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "无效的第一个待加倍项")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // 将错误更改为我们的新类型。
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // 这里也更新为新的错误类型。
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("第一个数的两倍是 {}", n),
        Err(e) => println!("错误：{}", e),
    }
}

#[test]
fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

// ? 的其他用途
// 注意在前面的例子中，我们对调用 parse 的直接反应是将库错误通过 map 转换为一个装箱的错误：
//
// .and_then(|s| s.parse::<i32>())
// .map_err(|e| e.into())
// 由于这是一个简单且常见的操作，如果能够省略就会很方便。
// 可惜的是，由于 and_then 不够灵活，所以无法实现这一点。不过，我们可以使用 ? 来替代。
//
// 之前我们将 ? 解释为 unwrap 或 return Err(err)。
// 这只是大致正确。实际上，它的含义是 unwrap 或 return Err(From::from(err))。
// 由于 From::from 是不同类型之间的转换工具，这意味着如果你在错误可转换为返回类型的地方使用 ?，它将自动进行转换。
//
// 在这里，我们使用 ? 重写了前面的例子。当为我们的错误类型实现 From::from 后，map_err 就不再需要了：

use std::error;
use std::fmt;

// 将别名改为使用 `Box<dyn error::Error>`。
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "无效的第一个待加倍项")
    }
}

impl error::Error for EmptyVec {}

// 结构与之前相同，但不再链式处理所有的 `Result` 和 `Option`，
// 而是使用 `?` 立即获取内部值。
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("第一个数的两倍是 {}", n),
        Err(e) => println!("错误：{}", e),
    }
}

#[wjj_lib::gen_test]
fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}


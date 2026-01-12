// 使用 Box 将错误装箱
// 一种既能编写简洁代码又能保留原始错误信息的方法是使用 Box 将它们装箱。
// 这种方法的缺点是底层错误类型只能在运行时确定，而不是静态确定的。
//
// 标准库通过让 Box 实现从任何实现了 Error trait 的类型到 trait 对象 Box<Error> 的转换来帮助我们装箱错误，
// 这是通过 From 实现的。

use std::error;
use std::fmt;

// 将别名改为使用 `Box<dyn error::Error>`。
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "无效的第一个待加倍项")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // 转换为 Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // 转换为 Box
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

// 包装错误
// 除了将错误装箱外，另一种方法是将它们包装在你自定义的错误类型中。

use std::error;
use std::error::Error;
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // 我们将使用解析错误的实现来处理它们的错误。
    // 提供额外信息需要向类型添加更多数据。
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "请使用至少包含一个元素的向量"),
            // 包装的错误包含额外信息，可通过 source() 方法获取。
            DoubleError::Parse(..) =>
                write!(f, "提供的字符串无法解析为整数"),
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,
            // 错误原因是底层实现的错误类型。它被隐式地
            // 转换为 trait 对象 `&error::Error`。这是可行的，因为
            // 底层类型已经实现了 `Error` trait。
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

// 实现从 `ParseIntError` 到 `DoubleError` 的转换。
// 当需要将 `ParseIntError` 转换为 `DoubleError` 时，
// `?` 运算符会自动调用这个转换。
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    // 这里我们隐式使用了 `ParseIntError` 的 `From` 实现（我们在上面定义的）
    // 来创建一个 `DoubleError`。
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("第一个数的两倍是 {}", n),
        Err(e) => {
            println!("错误：{}", e);
            if let Some(source) = e.source() {
                println!("  错误原因：{}", source);
            }
        },
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


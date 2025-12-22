use anyhow::{Error, Result};
use std::fmt;
use std::fmt::Arguments;

pub trait Exception {
    fn error_code(&self) -> &'static str;
}

pub trait RawException: Exception {
    fn error_msg(&self) -> &'static str;
}

pub trait FmtException: Exception {
    fn error_fmt_temp(&self) -> &'static str;
}

struct RawErr {
    error_code: &'static str,
    error_msg: &'static str,
}

struct FmtErr<'a> {
    error_code: &'static str,
    error_fmt_temp: &'static str,
    error_args: &'a [&'a dyn fmt::Display],
}

impl fmt::Display for RawErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {}", self.error_code, self.error_msg)
    }
}
//
// impl fmt::Display for FmtErr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Error {}: {}", self.error_code, self.error_msg)
//     }
// }
//
//
// // 自定义错误类型
// #[derive(Debug)]
// struct MyCustomError {
//     code: &'static str,
//     message: &'static str,
// }
//
// // 实现 Display
// impl fmt::Display for MyCustomError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Error {}: {}", self.code, self.message)
//     }
// }

// impl MyCustomError {
//     // 从 RawError 构造
//     fn from_raw<E: RawError>(err: E) -> Self {
//         MyCustomError {
//             code: err.error_code(),
//             message: err.error_msg().to_string(),
//         }
//     }
//
//     // 从 FmtError + Arguments 构造
//     // fn from_fmt<E: FmtError>(err: E, args: Arguments) -> Self {
//     //
//     //     MyCustomError {
//     //         code: err.error_code(),
//     //         message: format!(err.error_fmt_msg(), args),
//     //     }
//     // }
// }
//
// // 实现 std::error::Error
// impl std::error::Error for MyCustomError {}
//
// // 模拟一个返回 anyhow::Error 的函数
// fn do_something() -> Result<()> {
//     Err(Error::new(MyCustomError {
//         code: "404",
//         message: "Resource not found".to_string(),
//     }))
// }

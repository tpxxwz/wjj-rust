use thiserror::Error;

// 模拟第三方错误类型（这里用 std::io::Error 代替 sqlx::Error）
use std::io;

// 定义应用错误类型
#[derive(Debug, Error)]
pub enum AppError {
    #[error("数据库错误: code={code}, 原因={source}")]
    DbError {
        code: u32,
        #[source]
        source: io::Error, // 这里可以换成 sqlx::Error
    },

    #[error("配置错误: code={code}, msg={msg}")]
    ConfigError { code: u32, msg: String },

    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("解析错误")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("自定义From错误")]
    CustomFromError { code: u32, msg: String },
    // thiserror 2.x 版本已经不支持了
    // #[error("带有堆栈信息的错误")]
    // ErrorWithBacktrace {
    //     #[from]
    //     source: std::str::Utf8Error,
    //     #[backtrace]
    //     backtrace: std::backtrace::Backtrace,
    // },
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::CustomFromError {
            code: 500,
            msg: err.to_string(),
        }
    }
}

fn custom_from() -> Result<(), AppError> {
    let err = io::Error::new(io::ErrorKind::NotFound, "数据库连接失败");
    Err(err.into()) // 会调用From的吗
}

// 模拟一个数据库操作，返回 DbError
fn do_db() -> Result<(), AppError> {
    let err = io::Error::new(io::ErrorKind::NotFound, "数据库连接失败");
    Err(AppError::DbError {
        code: 500,
        source: err,
    })
}

// 模拟一个配置加载操作，返回 ConfigError
fn load_config() -> Result<(), AppError> {
    Err(AppError::ConfigError {
        code: 400,
        msg: "配置文件缺失".to_string(),
    })
}

fn net_call() -> Result<(), AppError> {
    Err(AppError::NetworkError("xxx.com".into()))
}

fn parse() -> Result<(), AppError> {
    let _num: i32 = "abc".parse()?; // 自动转换成 MyError::ParseInt
    Ok(())
}

#[test]
fn main() {
    // 测试数据库错误
    match do_db() {
        Ok(_) => println!("数据库操作成功"),
        Err(e) => {
            println!("捕获错误: {}", e);
            // 如果需要访问字段，可以用模式匹配
            match e {
                AppError::DbError { code, source } => {
                    println!("详细信息 -> code={}, 原始错误={}", code, source);
                }
                _ => {}
            }
        }
    }

    // 测试配置错误
    match load_config() {
        Ok(_) => println!("配置加载成功"),
        Err(e) => {
            println!("捕获错误: {}", e);
            match e {
                AppError::ConfigError { code, msg } => {
                    println!("详细信息 -> code={}, msg={}", code, msg);
                }
                _ => {}
            }
        }
    }
}

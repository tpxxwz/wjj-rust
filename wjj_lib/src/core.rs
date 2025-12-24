use minijinja::Environment;
use once_cell::sync::Lazy;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::sync::RwLock;

pub trait Exc {
    fn err_code(&self) -> &'static str;
}

pub trait RawExc: Exc {
    fn err_msg(&self) -> &'static str;
}

#[derive(Debug)]
struct RawErr {
    err_code: &'static str,
    err_msg: &'static str,
}

impl RawErr {
    pub fn from_exc<E>(exc: &E) -> Self
    where
        E: RawExc,
    {
        RawErr {
            err_code: exc.err_code(),
            err_msg: exc.err_msg(),
        }
    }
}

pub trait FmtExc {
    fn err_code(&self) -> &'static str;
    fn err_tpl_name(&self) -> &'static str;
}

impl fmt::Display for RawErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {}", self.err_code, self.err_msg)
    }
}

pub static ENV: Lazy<RwLock<Environment<'static>>> = Lazy::new(|| {
    let env = Environment::new();
    RwLock::new(env)
});

pub fn render_template(name: &str, args: &serde_json::Value) -> Result<String, minijinja::Error> {
    let env = ENV.read().unwrap();
    let tpl = env.get_template(name).unwrap();
    tpl.render(args)
}

#[derive(Debug)]
pub struct FmtErr {
    pub err_code: &'static str,
    pub err_tpl_name: &'static str,
    pub err_args: serde_json::Value,
}

impl FmtErr {
    pub fn from_exc<E>(exc: &E, err_args: serde_json::Value) -> Self
    where
        E: FmtExc,
    {
        FmtErr {
            err_code: exc.err_code(),
            err_tpl_name: exc.err_tpl_name(),
            err_args,
        }
    }
}

impl fmt::Display for FmtErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match render_template(self.err_tpl_name, &self.err_args) {
            Ok(s) => f.write_str(&s),
            Err(e) => {
                // 你可以 log，也可以忽略
                eprintln!("template render failed: {e}");
                Err(fmt::Error)
            }
        }
    }
}

impl Error for RawErr {}
impl Error for FmtErr {}

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

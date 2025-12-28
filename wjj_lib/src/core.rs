use minijinja::{Environment, Template};
use once_cell::sync::Lazy;
use std::error::Error;
use std::{fmt, fs};
use std::fmt::Debug;
use std::sync::Arc;
use serde_json::json;

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

impl fmt::Display for RawErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {}", self.err_code, self.err_msg)
    }
}

pub trait FmtExc {
    fn err_code(&self) -> &'static str;
    fn err_tpl(&self) -> &'static str;
}

pub static ENV: Lazy<Arc<Environment<'static>>> = Lazy::new(|| Arc::new(Environment::new()));

#[derive(Debug)]
pub struct FmtErr<'env: 'source, 'source> {
    pub err_code: &'static str,
    pub err_tpl: &'static str,
    pub template: Template<'env, 'source>,
    pub err_args: serde_json::Value,
}

impl FmtErr<'static, 'static> {
    pub fn from_exc<E>(exc: &E, err_args: serde_json::Value) -> Self
    where
        E: FmtExc,
    {
        FmtErr {
            err_code: exc.err_code(),
            err_tpl: exc.err_tpl(),
            template: ENV.template_from_str(exc.err_tpl()).unwrap(),
            err_args,
        }
    }
}

impl fmt::Display for FmtErr<'static, 'static> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self
            .template
            .render(self.err_args.clone())
            .unwrap_or_else(|_| format!("[模板渲染失败: {}]", self.err_code));
        write!(f, "{}", output)
    }
}

impl Error for RawErr {}
impl Error for FmtErr<'static, 'static> {}




// 错误元信息结构
pub struct ErrorMeta {
    pub code: &'static str,
    pub template: &'static str,
}

// 枚举定义错误类型
#[derive(Debug)]
pub enum AppError {
    IoError,
    DbError,
    NotFound,
}

// 每个错误对应的元信息
pub const IO_ERROR_META: ErrorMeta = ErrorMeta {
    code: "E2001",
    template: "IO错误: 文件 {{ filename }} 读取失败",
};

pub const DB_ERROR_META: ErrorMeta = ErrorMeta {
    code: "E3001",
    template: "数据库错误: 查询 {{ query }} 执行失败",
};

pub const NOT_FOUND_META: ErrorMeta = ErrorMeta {
    code: "E404",
    template: "资源 {{ resource }} 未找到",
};

// 枚举方法返回对应的元信息
impl AppError {
    pub fn meta(&self) -> &'static ErrorMeta {
        match self {
            AppError::IoError => &IO_ERROR_META,
            AppError::DbError => &DB_ERROR_META,
            AppError::NotFound => &NOT_FOUND_META,
        }
    }
}


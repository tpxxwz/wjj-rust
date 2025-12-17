use anyhow::{Context, Error, Result};
use std::fmt::{Display, Formatter};
// use std::fmt::Display;
//
// pub trait Exception {}
// pub struct SysErr {}
// pub struct BizErr {}
// pub enum WjjError {
//     SystemErr(dyn Exception),
//     BusinessErr(dyn Exception),
// }
//
// pub trait ContextExt<T, E> {
//     fn context_obj<C>(self, obj: C) -> Result<T, Error>
//     where
//         C: Display + Send + Sync + 'static;
// }
//
// impl<T, E> ContextExt<T, E> for Result<T, E>
// where
//     E: std::error::Error + Send + Sync + 'static,
// {
//     fn context_obj<C>(self, obj: C) -> Result<T, Error>
//     where
//         C: Display + Send + Sync + 'static,
//     {
//         self.with_context(|| self.with_context(|| obj) // 自动转换成闭包
//     }
// }

// format_args!("Hello {}, age {}", "Rust", 18)

pub struct Exception {}

impl Display for Exception {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub trait ErrorCode {
    fn error_code(&self) -> &'static str;
}

pub trait RawError: ErrorCode {
    fn error_msg(&self) -> &'static str;
}

pub trait FmtError: ErrorCode {
    fn error_fmt_msg(&self) -> &'static str;
    fn error_fmt_desc(&self) -> &'static str;
}

pub trait ContextExt<T, E>: Context<T, E> {
    fn with_raw_error<C>(self, context: C) -> Result<T>
    where
        C: RawError;
    fn with_fmt_error<C>(self, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static;
}

impl<T, E> ContextExt<T, E> for Result<T, E>
where
    Result<T, E>: Context<T, E>,
    E: Display + Send + Sync + 'static,
{
    fn with_raw_error<C>(self, context: C) -> Result<T>
    where
        C: RawError,
    {
        self.with_context(|| Exception)
    }
    fn with_fmt_error<C>(self, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
    {
        println!("Logging: {}", context);
        self.context(context)
    }
}

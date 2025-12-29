// use minijinja::{Environment, Template};
// use once_cell::sync::Lazy;
// use serde_json::json;
// use std::error::Error;
// use std::fmt::Debug;
// use std::sync::Arc;
// use std::{fmt, fs};
//
// pub trait Exc {
//     fn err_code(&self) -> &'static str;
// }
//
// pub trait RawExc: Exc {
//     fn err_msg(&self) -> &'static str;
// }
//
// #[derive(Debug)]
// struct RawErr {
//     err_code: &'static str,
//     err_msg: &'static str,
// }
//
// impl RawErr {
//     pub fn from_exc<E>(exc: &E) -> Self
//     where
//         E: RawExc,
//     {
//         RawErr {
//             err_code: exc.err_code(),
//             err_msg: exc.err_msg(),
//         }
//     }
// }
//
// impl fmt::Display for RawErr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Error {}: {}", self.err_code, self.err_msg)
//     }
// }
//
//
// pub trait FmtExc {
//     fn err_code(&self) -> &'static str;
//     fn err_tpl(&self) -> &'static str;
// }
//
// #[derive(Debug)]
// pub struct FmtErr {
//     pub err_code: &'static str,
//     pub err_args: serde_json::Value,
// }
//
// impl FmtErr {
//     pub fn from_exc<E>(exc: E, err_args: serde_json::Value) -> Self
//     where
//         E: FmtExc,
//     {
//         FmtErr {
//             err_code: exc.err_code(),
//             err_args,
//         }
//     }
// }
//
// impl fmt::Display for FmtErr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let tpl = match ENV.get_template(self.err_code) {
//             Ok(t) => t,
//             Err(_) => {
//                 return write!(f, "[template not exists: {}]", self.err_code);
//             }
//         };
//
//         let output = tpl
//             .render(self.err_args.clone())
//             .unwrap_or_else(|_| format!("[template render failed: {}]", self.err_code));
//
//         write!(f, "{}", output)
//     }
// }
//
// impl Error for RawErr {}
// impl Error for FmtErr {}
//
//
// #[derive(Debug, Clone, Copy)]
// pub struct Row {
//     pub a: &'static str,
//     pub i: &'static str,
//     pub u: &'static str,
//     pub e: &'static str,
//     pub o: &'static str,
//     pub warping: bool,
// }
//
// pub static ENV: Lazy<Arc<Environment<'static>>> = Lazy::new(|| Arc::new(Environment::new()));
//
// // 定义 TemplateRegistration
// pub struct TemplateRegistration {
//     pub f: fn(&mut Environment<'static>),
// }
//
// inventory::collect!(TemplateRegistration);

use common_macros::{FmtErr, RawErr};
use minijinja::{Environment, UndefinedBehavior};
use once_cell::sync::Lazy;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use linkme::distributed_slice;

pub struct TemplateRegistration {
    pub f: fn(&mut Environment<'static>),
}
#[distributed_slice]
pub static TEMPLATE_REGISTRATIONS: [TemplateRegistration] = [..];

pub static ENV: Lazy<Arc<Environment<'static>>> = Lazy::new(|| {
    let mut env = Environment::new();
    env.set_undefined_behavior(UndefinedBehavior::Strict);
    // 自动收集所有插件注册的模板
    for reg in TEMPLATE_REGISTRATIONS {
        (reg.f)(&mut env);
    }
    Arc::new(env)
});

#[derive(Debug)]
pub struct RawErr {
    pub err_code: &'static str,
    pub err_msg: &'static str,
}

#[derive(Debug)]
pub struct FmtErr {
    pub err_code: &'static str,
    pub err_args: serde_json::Value,
}

impl fmt::Display for RawErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err_msg)
    }
}

impl fmt::Display for FmtErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = ENV
            .get_template(self.err_code)
            .and_then(|tpl| tpl.render(self.err_args.clone()))
            .unwrap_or_else(|_| format!("[render failed. code: {}]", self.err_code));
        f.write_str(&output)
    }
}

impl Error for RawErr {}

impl Error for FmtErr {}

#[derive(RawErr)]
#[err_code_prefix = "999"]
pub enum BaseRawErrs {
    #[error(err_code = "00000", err_msg = "System Error")]
    SysRawErr,
}

#[derive(FmtErr)]
#[err_code_prefix = "999"]
pub enum BaseFmtErrs {
    #[error(err_code = "10000", err_tpl = "System Error, cause: {{ cause }}")]
    SysFmtErr,
}

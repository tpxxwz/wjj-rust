extern crate self as common_core;

mod err;

pub use common_macros::{fmt_err, raw_err};
pub use err::{
    BaseFmtErrs, BaseRawErrs, ERR_CODE_REGISTRATIONS, ErrCodeRegistration, FmtErr, RawErr,
    TEMPLATE_REGISTRATIONS, TemplateRegistration,
};
pub use minijinja::Environment;
pub use serde_json::Value;

#[ctor::ctor]
fn init_common_core() {
    err::init();
}

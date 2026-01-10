extern crate self as common_core;

mod err;

pub use common_macros::{FmtErr, RawErr};
pub use err::{
    BaseFmtErrs, BaseRawErrs, FmtErr, RawErr, TEMPLATE_REGISTRATIONS, TemplateRegistration,
};
pub use minijinja::Environment;
pub use serde_json::Value;

#[ctor::ctor]
fn init_common_core() {
    let _ = &*err::ENV;
}

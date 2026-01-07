extern crate self as common_core;

mod err;

pub use crate::err::FmtErr;
pub use crate::err::RawErr;
pub use err::BaseFmtErrs;
pub use err::BaseRawErrs;
pub use err::TemplateRegistration;

inventory::collect!(TemplateRegistration);

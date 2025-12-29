mod env;

pub use env::ENV;
pub use env::TemplateRegistration;

inventory::collect!(TemplateRegistration);
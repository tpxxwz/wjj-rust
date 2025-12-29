use minijinja::Environment;
use once_cell::sync::Lazy;
use std::sync::Arc;

pub struct TemplateRegistration {
    pub f: fn(&mut Environment<'static>),
}

pub static ENV: Lazy<Arc<Environment<'static>>> = Lazy::new(|| {
    let mut env = Environment::new();

    // 自动收集所有插件注册的模板
    for reg in inventory::iter::<TemplateRegistration> {
        (reg.f)(&mut env);
    }

    Arc::new(env)
});
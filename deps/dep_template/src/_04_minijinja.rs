use minijinja::{Environment, Template};
use serde_json::json;
use std::error::Error;

#[test]
fn main() -> Result<(), Box<dyn Error>> {
    let mut env = Environment::new();

    // 1. 直接渲染字符串模板（等价于 Handlebars 的 render_template）
    let tpl: Template = env.template_from_str("Hello {{ name }}")?;
    println!("{}", tpl.render(&json!({"name": "foo"}))?);

    // 2. 注册模板（等价于 register_template_string）
    env.add_template("tpl_1", "Good afternoon, {{ name }}")?;

    // 3. 用模板名渲染（等价于 render("tpl_1", ...)）
    let tpl = env.get_template("tpl_1")?;
    println!("{}", tpl.render(&json!({"name": "foo"}))?);

    env.add_template("tpl_3", "Good afternoon")?;
    let tpl = env.get_template("tpl_3")?;
    println!("{}", tpl.render(json!({}))?);
    Ok(())
}

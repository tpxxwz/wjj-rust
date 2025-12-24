use serde_json::json;
use std::error::Error;
use tera::{Context, Tera};

#[test]
fn main() -> Result<(), Box<dyn Error>> {
    // 创建 Tera 实例（可变）
    let mut tera = Tera::default();

    // 1. 直接渲染字符串模板（使用 serde_json::Value）
    let tpl = "Hello {{ name }}";

    let data = json!({
        "name": "foo"
    });

    // 关键：Context::from_value
    let ctx = Context::from_value(data)?;

    let out = tera.render_str(tpl, &ctx)?;
    println!("{}", out);

    // 2. 注册模板
    tera.add_raw_template("tpl_1", "Good afternoon, {{ name }}")?;

    // 3. 用模板名渲染（同样用 JSON）
    let data2 = json!({
        "name": "foo"
    });

    let ctx2 = Context::from_value(data2)?;

    let out2 = tera.render("tpl_1", &ctx2)?;
    println!("{}", out2);

    Ok(())
}

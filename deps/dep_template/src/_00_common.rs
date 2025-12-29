use common_macros::register_template;

// 自动注册模板
register_template!(("hello", "Hello {{ name }}!"));
register_template!(("bye", "Bye {{ name }}!"));

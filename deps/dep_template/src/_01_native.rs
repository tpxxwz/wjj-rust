use std::fmt::Write;
use std::fmt;

fn format_dyn(tpl: &str, args: &[(&str, &dyn fmt::Display)]) -> String {
    let mut out = String::new();

    let mut rest = tpl;
    for (k, v) in args {
        if let Some(pos) = rest.find("{}") {
            out.push_str(&rest[..pos]);
            write!(out, "{}", v).unwrap();
            rest = &rest[pos + 2..];
        }
    }
    out.push_str(rest);
    out
}

#[test]
fn demo_dyn_fmt() {
    let s = format_dyn("Hello {}, id={}", &[("name", &"foo"), ("id", &42)]);

    assert_eq!(s, "Hello foo, id=42");
}

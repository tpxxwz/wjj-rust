// ===== 示例 1：借用版（有生命周期）=====

struct FmtErr<'a> {
    error_code: &'static str,
    error_fmt_temp: &'static str,
    error_args: &'a String, // 借用
}

fn make_err<'a>(args: &'a String) -> FmtErr<'a> {
    FmtErr {
        error_code: "E001",
        error_fmt_temp: "hello {}",
        error_args: args,
    }
}

#[test]
fn demo_borrow_ok() {
    let s = String::from("foo");

    let err = make_err(&s); // ✅ s 还活着

    assert_eq!(err.error_args.as_str(), "foo");
}

// 下面这个 test **故意写成不合法**
// 👉 你把注释去掉，编译器会直接报错

// #[test]
// fn demo_borrow_error() {
//     let err;
// 
//     {
//         let s = String::from("foo");
//         err = make_err(&s); // ❌ 借用活得比 s 久
//     } // s 在这里被 drop
//
//     println!("{}", err.error_args);
// }


// ===== 示例 2：拥有版（没有生命周期）=====

struct FmtErr1 {
    error_code: &'static str,
    error_fmt_temp: &'static str,
    error_args: String, // 自己拥有
}

fn make_err1(args: String) -> FmtErr1 {
    FmtErr1 {
        error_code: "E001",
        error_fmt_temp: "hello {}",
        error_args: args, // move
    }
}

#[test]
fn demo_owned_ok() {
    let err;

    {
        let s = String::from("foo");
        err = make_err1(s); // ✅ 所有权转移
    } // s 已经被 move 掉了

    assert_eq!(err.error_args.as_str(), "foo");
}

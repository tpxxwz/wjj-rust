// 作为输出参数
// 既然闭包可以作为输入参数，那么将闭包作为输出参数返回也应该是可行的。
// 然而，匿名闭包类型本质上是未知的，因此我们必须使用 impl Trait 来返回它们。
//
// 可用于返回闭包的有效 trait 包括：
//
// Fn
// FnMut
// FnOnce
// 此外，必须使用 move 关键字，它表示所有捕获都是按值进行的。
// 这是必要的，因为任何通过引用捕获的变量都会在函数退出时被丢弃，从而在闭包中留下无效的引用。

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("这是一个：{}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("这是一个：{}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("这是一个：{}", text)
}

#[wjj_lib::gen_test]
fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
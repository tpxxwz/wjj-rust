// 输入函数
// 既然闭包可以作为参数使用，你可能会想知道函数是否也可以这样。
// 确实可以！如果你声明一个函数，它接受一个闭包作为参数，那么任何满足该闭包 trait 约束的函数都可以作为参数传递。

// 定义一个函数，它接受一个由 `Fn` 约束的泛型参数 `F`，并调用它
fn call_me<F: Fn()>(f: F) {
    f();
}

// 定义一个满足 `Fn` 约束的包装函数
fn function() {
    println!("我是函数！");
}

#[test]
fn main() {
    // 定义一个满足 `Fn` 约束的闭包
    let closure = || println!("我是闭包！");

    call_me(closure);
    call_me(function);
}
// 另外需要注意的是，Fn、FnMut 和 FnOnce 这些 trait 决定了闭包如何从外部作用域捕获变量。

// 自定义
// 一些条件（如 target_os）是由 rustc 隐式提供的，但自定义条件必须通过 --cfg 标志传递给 rustc。

#[cfg(some_condition)]
fn conditional_function() {
    println!("条件满足！");
}

#[test]
fn main() {
    // 下一行的注释解开 用rustc --cfg some_condition的命令执行
    // conditional_function();
}

// 尝试运行这段代码，看看没有自定义 cfg 标志会发生什么。
//
// 使用自定义 cfg 标志：
// $ rustc --cfg some_condition _03_cfg_11_custom.rs && ./_03_cfg_11_custom
// condition met!

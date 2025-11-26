// cfg
// 配置条件检查可以通过两种不同的操作符实现：
//
// cfg 属性：在属性位置使用 #[cfg(...)]
// cfg! 宏：在布尔表达式中使用 cfg!(...)
// 前者启用条件编译，后者在运行时条件性地求值为 true 或 false 字面量，允许在运行时进行检查。两者使用相同的参数语法。
//
// cfg! 与 #[cfg] 不同，它不会移除任何代码，只会求值为 true 或 false。
// 例如，当 cfg! 用于条件时，if/else 表达式中的所有代码块都需要是有效的，无论 cfg! 正在评估什么。


// 这个函数只有在目标操作系统是 linux 时才会被编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("你正在运行 Linux！");
}

// 而这个函数只有在目标操作系统**不是** Linux 时才会被编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("你**不是**在运行 Linux！");
}

#[wjj_lib::gen_test]
fn main() {
    are_you_on_linux();

    println!("你确定吗？");
    if cfg!(target_os = "linux") {
        println!("是的，这绝对是 Linux！");
    } else {
        println!("是的，这绝对**不是** Linux！");
    }
}


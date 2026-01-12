// 程序参数
// 标准库
// 可以使用 std::env::args 访问命令行参数，它返回一个迭代器，为每个参数生成一个 String：

use std::env;

#[test]
fn main() {
    let args: Vec<String> = env::args().collect();

    // 第一个参数是用于调用程序的路径
    println!("我的路径是 {}。", args[0]);

    // 其余参数是传递的命令行参数
    // 像这样调用程序：
    //   $ ./args arg1 arg2
    println!("我获得了 {:?} 个参数：{:?}。", args.len() - 1, &args[1..]);
}

// $ ./_07_program_arguments 1 2 3
// 程序路径：./args
// 接收到 3 个参数：["1"、"2"、"3"]
// Crates
// 此外，在开发命令行应用程序时，还有许多 crate 可以提供额外的功能。其中，clap 是一个广受欢迎的命令行参数处理 crate。

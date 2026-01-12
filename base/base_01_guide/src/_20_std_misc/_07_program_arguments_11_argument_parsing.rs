// 参数解析
// 可以使用模式匹配来解析简单的参数：

use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!(
        "用法：
match_args <字符串>
    检查给定的字符串是否为正确答案。
match_args {{increase|decrease}} <整数>
    将给定的整数增加或减少 1。"
    );
}

#[test]
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // 未传递参数
        1 => {
            println!("我的名字是 'match_args'。试试传递一些参数吧！");
        }
        // 传递了一个参数
        2 => match args[1].parse() {
            Ok(42) => println!("这就是正确答案！"),
            _ => println!("这不是正确答案。"),
        },
        // 传递了一个命令和一个参数
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // 解析数字
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("错误：第二个参数不是整数");
                    help();
                    return;
                }
            };
            // 解析命令
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("错误：无效的命令");
                    help();
                }
            }
        }
        // 所有其他情况
        _ => {
            // 显示帮助信息
            help();
        }
    }
}

// 如果你将程序命名为 match_args.rs 并使用 rustc match_args.rs 编译它，你可以按以下方式执行：
//
// $ ./match_args Rust
// This is not the answer.
// $ ./match_args 42
// This is the answer!
// $ ./match_args do something
// error: second argument not an integer
// usage:
// match_args <string>
// Check whether given string is the answer.
// match_args {increase|decrease} <integer>
// Increase or decrease given integer by one.
// $ ./match_args do 42
// error: invalid command
// usage:
// match_args <string>
// Check whether given string is the answer.
// match_args {increase|decrease} <integer>
// Increase or decrease given integer by one.
// $ ./match_args increase 42
// 43

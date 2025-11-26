// 管道
// std::process::Child 结构体代表一个子进程，它暴露了 stdin、stdout 和 stderr 句柄，
// 允许通过管道与底层进程进行交互。

use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str =
    "the quick brown fox jumps over the lazy dog\n";

#[wjj_lib::gen_test]
fn main() {
    // 启动 `wc` 命令
    let mut cmd = if cfg!(target_family = "windows") {
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command").arg("$input | Measure-Object -Line -Word -Character");
        cmd
    } else {
        Command::new("wc")
    };
    let process = match cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Err(why) => panic!("无法启动 wc：{}", why),
        Ok(process) => process,
    };

    // 向 `wc` 的 `stdin` 写入字符串。
    //
    // `stdin` 的类型是 `Option<ChildStdin>`，但我们知道这个实例
    // 必定存在，所以可以直接 `unwrap` 它。
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("无法写入 wc 的标准输入：{}", why),
        Ok(_) => println!("已将 pangram 发送给 wc"),
    }

    // 由于 `stdin` 在上述调用后不再存活，它会被 `drop`，
    // 管道随之关闭。
    //
    // 这一点非常重要，否则 `wc` 不会开始处理
    // 我们刚刚发送的输入。

    // `stdout` 字段的类型也是 `Option<ChildStdout>`，因此必须解包。
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("无法读取 wc 的标准输出：{}", why),
        Ok(_) => print!("wc 的响应为：\n{}", s),
    }
}


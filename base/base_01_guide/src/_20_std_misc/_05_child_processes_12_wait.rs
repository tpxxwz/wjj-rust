// 等待
// 如果你想等待一个 process::Child 完成，你必须调用 Child::wait，它会返回一个 process::ExitStatus。

use std::process::Command;

#[test]
fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("到达 main 函数末尾");
}
// $ rustc _05_child_processes_12_wait.rs && ./_05_child_processes_12_wait
// # `wait` 会持续运行 5 秒，直到 `sleep 5` 命令执行完毕
// reached end of main

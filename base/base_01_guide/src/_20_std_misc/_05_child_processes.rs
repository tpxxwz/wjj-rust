// 子进程
// process::Output 结构体表示已完成子进程的输出，而 process::Command 结构体是一个进程构建器。

use std::process::Command;

#[test]
fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("执行进程失败：{}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc 执行成功，标准输出为：\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc 执行失败，标准错误输出为：\n{}", s);
    }
}

// （建议您尝试在上述示例中向 rustc 传递一个错误的标志）

// open
// open 函数可用于以只读模式打开文件。
//
// File 拥有一个资源（即文件描述符），并在被 drop 时负责关闭文件。

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[wjj_lib::gen_test]
fn main() {
    // 创建指向目标文件的路径
    let path = Path::new("hello.txt");
    let display = path.display();

    // 以只读模式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("无法打开 {}: {}", display, why),
        Ok(file) => file,
    };

    // 将文件内容读入字符串，返回 `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("无法读取 {}: {}", display, why),
        Ok(_) => print!("{} 的内容：\n{}", display, s),
    }

    // `file` 超出作用域，"hello.txt" 文件随之关闭
}
// 以下是预期的成功输出：
//
// $ echo "Hello World!" > hello.txt
// $ rustc open.rs && ./open
// hello.txt 的内容：
// Hello World!
// （建议您在不同的失败情况下测试上述示例：例如 hello.txt 不存在，或 hello.txt 不可读等。）




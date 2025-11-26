// 路径
// Path 结构体表示底层文件系统中的文件路径。
// Path 有两种变体：用于类 UNIX 系统的 posix::Path 和用于 Windows 的 windows::Path。
// prelude 会导出适合特定平台的 Path 变体。
//
// Path 可以从 OsStr 创建，并提供多种方法来获取路径所指向的文件或目录的信息。
//
// Path 是不可变的。Path 的所有权版本是 PathBuf。
// Path 和 PathBuf 之间的关系类似于 str 和 String：PathBuf 可以原地修改，并且可以解引用为 Path。
//
// 注意，Path 在内部并非表示为 UTF-8 字符串，而是存储为 OsString。
// 因此，将 Path 转换为 &str 并非零开销操作，且可能失败（返回一个 Option）。
// 然而，Path 可以自由地转换为 OsString 或 &OsStr，分别使用 into_os_string 和 as_os_str 方法。

use std::path::Path;

#[wjj_lib::gen_test]
fn main() {
    // 从 `&'static str` 创建一个 `Path`
    let path = Path::new(".");

    // `display` 方法返回一个可用于显示的结构体
    let _display = path.display();

    // `join` 使用操作系统特定的分隔符将路径与字节容器合并，
    // 并返回一个 `PathBuf`
    let mut new_path = path.join("a").join("b");

    // `push` 使用 `&Path` 扩展 `PathBuf`
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name` 更新 `PathBuf` 的文件名
    new_path.set_file_name("package.tgz");

    // 将 `PathBuf` 转换为字符串切片
    match new_path.to_str() {
        None => panic!("新路径不是有效的 UTF-8 序列"),
        Some(s) => println!("新路径是 {}", s),
    }
}
// 请务必查看其他 Path 方法（posix::Path 或 windows::Path）以及 Metadata 结构体。


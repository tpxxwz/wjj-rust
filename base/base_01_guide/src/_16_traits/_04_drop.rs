// Drop
// Drop trait 只有一个方法：drop，它会在对象离开作用域时自动调用。
// Drop trait 的主要用途是释放实现该 trait 的实例所拥有的资源。
//
// Box、Vec、String、File 和 Process 是一些实现了 Drop trait 以释放资源的类型示例。
// 你也可以为任何自定义数据类型手动实现 Drop trait。
//
// 下面的例子在 drop 函数中添加了一个控制台打印，用于宣告它被调用的时机。

struct Droppable {
    name: &'static str,
}

// 这个简单的 `drop` 实现添加了一个控制台打印
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> 正在释放 {}", self.name);
    }
}

#[wjj_lib::gen_test]
fn main1() {
    let _a = Droppable { name: "a" };

    // 块 A
    {
        let _b = Droppable { name: "b" };

        // 块 B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("正在退出块 B");
        }
        println!("刚刚退出了块 B");

        println!("正在退出块 A");
    }
    println!("刚刚退出了块 A");

    // 可以使用 `drop` 函数手动释放变量
    drop(_a);
    // TODO ^ 试试注释掉这一行

    println!("main 函数结束");

    // `_a` 在这里**不会**被再次 `drop`，因为它已经
    // 被（手动）`drop` 过了
}


// 下面是一个更实用的例子，展示如何使用 Drop 特质在不再需要临时文件时自动清理它们：

use std::fs::File;
use std::path::PathBuf;

struct TempFile {
    file: File,
    path: PathBuf,
}

impl TempFile {
    fn new(path: PathBuf) -> std::io::Result<Self> {
        // 注意：File::create() 会覆盖现有文件
        let file = File::create(&path)?;

        Ok(Self { file, path })
    }
}

// When TempFile is dropped:
// 1. First, our drop implementation will remove the file's name from the filesystem.
// 2. Then, File's drop will close the file, removing its underlying content from the disk.
impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_file(&self.path) {
            eprintln!("删除临时文件失败：{}", e);
        }
        println!("> 已丢弃临时文件：{:?}", self.path);
        // File's drop is implicitly called here because it is a field of this struct.
    }
}

#[wjj_lib::gen_test]
fn main2() -> std::io::Result<()> {
    // 创建新作用域来演示丢弃行为
    {
        let temp = TempFile::new("test.txt".into())?;
        println!("已创建临时文件");
        // 当 temp 超出作用域时，文件会自动清理
    }
    println!("作用域结束 - 文件应该被清理");

    // 如果需要，我们也可以手动丢弃
    let temp2 = TempFile::new("another_test.txt".into())?;
    drop(temp2); // 显式丢弃文件
    println!("手动丢弃的文件");

    Ok(())
}


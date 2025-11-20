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

fn main() -> std::io::Result<()> {
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

crate::gen_test!(main);
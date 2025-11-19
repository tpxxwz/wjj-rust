// 测试
// 众所周知，测试是任何软件不可或缺的一部分！
// Rust 为单元测试和集成测试提供了一流的支持（参见《Rust 程序设计语言》中的这一章）。
//
// 从上面链接的测试章节中，我们了解了如何编写单元测试和集成测试。
// 在组织结构上，我们可以将单元测试放在它们所测试的模块中，而将集成测试放在专门的 tests/ 目录中：
// foo
// ├── Cargo.toml
// ├── src
// │   └── main.rs
// │   └── lib.rs
// └── tests
// ├── my_test.rs
// └── my_other_test.rs
// tests 目录中的每个文件都是一个独立的集成测试，即旨在测试你的库，就像它被依赖的 crate 调用一样。
//
// 测试章节详细阐述了三种不同的测试风格：单元测试、文档测试和集成测试。
//
// cargo 自然提供了一种简便的方式来运行所有测试！
// $ cargo test
// 你将看到类似这样的输出：
// $ cargo test
// Compiling blah v0.1.0 (file:///nobackup/blah)
// Finished dev [unoptimized + debuginfo] target(s) in 0.89 secs
// Running target/debug/deps/blah-d3b32b97275ec472
//
// running 4 tests
// test test_bar ... ok
// test test_baz ... ok
// test test_foo_bar ... ok
// test test_foo ... ok
//
// test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//
// 你还可以运行名称匹配特定模式的测试：
// $ cargo test test_foo
// $ cargo test test_foo
// Compiling blah v0.1.0 (file:///nobackup/blah)
// Finished dev [unoptimized + debuginfo] target(s) in 0.35 secs
// Running target/debug/deps/blah-d3b32b97275ec472
//
// running 2 tests
// test test_foo ... ok
// test test_foo_bar ... ok
//
// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
// 需要注意：Cargo 可能会并发运行多个测试，因此请确保它们之间不会产生竞态条件。
//
// 并发可能导致问题的一个例子是，如果两个测试同时输出到同一个文件，如下所示：
#[cfg(test)]
mod tests {
    // 导入必要的模块
    use std::fs::OpenOptions;
    use std::io::Write;

    // 这个测试向文件写入内容
    #[test]
    fn test_file() {
        // 打开 ferris.txt 文件，如果不存在则创建一个
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("无法打开 ferris.txt");

        // 循环打印 "Ferris" 5 次
        for _ in 0..5 {
            file.write_all("Ferris\n".as_bytes())
                .expect("无法写入 ferris.txt");
        }
    }

    // 这个测试尝试写入同一个文件
    #[test]
    fn test_file_also() {
        // 打开 ferris.txt 文件，如果不存在则创建一个
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("无法打开 ferris.txt");

        // 循环打印 "Corro" 5 次
        for _ in 0..5 {
            file.write_all("Corro\n".as_bytes())
                .expect("无法写入 ferris.txt");
        }
    }
}
// 尽管预期结果应该是：
// $ cat ferris.txt
// Ferris
// Ferris
// Ferris
// Ferris
// Ferris
// Corro
// Corro
// Corro
// Corro
// Corro
// 但实际写入 ferris.txt 的内容可能是这样的：
// $ cargo test test_file && cat ferris.txt
// Corro
// Ferris
// Corro
// Ferris
// Corro
// Ferris
// Corro
// Ferris
// Corro
// Ferris

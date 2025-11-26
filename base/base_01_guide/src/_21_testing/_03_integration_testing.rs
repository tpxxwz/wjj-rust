// 集成测试
// 单元测试每次只隔离测试一个模块：它们规模小，可以测试私有代码。集成测试则位于 crate 外部，仅使用其公共接口，就像其他代码一样。集成测试的目的是验证库的多个部分能否正确协同工作。
//
// Cargo 在 src 目录旁的 tests 目录中查找集成测试。
//
// 文件 src/lib.rs：

// 在名为 `adder` 的 crate 中定义此内容。
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
// 测试文件：tests/integration_test.rs：
// #[test]
// fn test_add() {
//     assert_eq!(adder::add(3, 2), 5);
// }

// 使用 cargo test 命令运行测试：
// $ cargo test
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//
// Running target/debug/deps/integration_test-bcd60824f5fbfe19
//
// running 1 test
// test test_add ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//
// Doc-tests adder
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

// tests 目录中的每个 Rust 源文件都被编译为独立的 crate。为了在集成测试之间共享代码，我们可以创建一个包含公共函数的模块，然后在测试中导入并使用它。
//
// 文件 tests/common/mod.rs：
pub fn setup() {
    // 一些设置代码，如创建必要的文件/目录，启动
    // 服务器等。
}

// 测试文件：tests/integration_test.rs
// 导入 common 模块。
// mod common;
//
// #[test]
// fn test_add() {
//     // 使用公共代码。
//     common::setup();
//     assert_eq!(adder::add(3, 2), 5);
// }

// 将模块创建为 tests/common.rs 也可行，但不推荐，因为测试运行器会将该文件视为测试 crate 并尝试运行其中的测试。
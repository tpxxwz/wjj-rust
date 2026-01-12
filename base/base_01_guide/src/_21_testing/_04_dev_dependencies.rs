// 开发依赖
// 有时我们需要仅用于测试（或示例、基准测试）的依赖项。这些依赖项添加在 Cargo.toml 的 [dev-dependencies] 部分。
// 这些依赖项不会传递给依赖于本包的其他包。
//
// 例如 pretty_assertions，它扩展了标准的 assert_eq! 和 assert_ne! 宏，提供彩色差异对比。
// 文件 Cargo.toml：

// 省略标准的 crate 数据
// [dev-dependencies]
// pretty_assertions = "1"

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // 仅用于测试的 crate。不能在非测试代码中使用。

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}

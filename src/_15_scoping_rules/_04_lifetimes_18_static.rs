// 静态
// Rust 有几个保留的生命周期名称。其中之一是 'static。你可能在两种情况下遇到它
// 具有 'static 生命周期的引用：
// let s: &'static str = "你好，世界";
//
// 'static 作为 trait 约束的一部分：
// fn generic<T>(x: T) where T: 'static {}
//
// 这两种情况虽然相关但有微妙的区别，这也是学习 Rust 时常见的困惑来源。以下是每种情况的一些例子：
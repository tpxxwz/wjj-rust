// 属性
// 属性是应用于模块、crate 或条目的元数据。这些元数据可用于以下目的：
//
// 代码的条件编译
// 设置 crate 的名称、版本和类型（二进制或库）
// 禁用 代码检查（警告）
// 启用编译器特性（如宏、全局导入等）
// 链接外部库
// 将函数标记为单元测试
// 将函数标记为基准测试的一部分
// 类属性宏

// #![inner_attribute] 应用于包含它的条目（通常是模块或 crate）。
// 换句话说，这种属性被解释为应用于它所在的整个作用域。
// 以下是一个示例，其中 #![allow(unused_variables)] 应用于整个 crate（如果放置在 main.rs 中）：
#![allow(unused_variables)]

#[wjj_lib::gen_test]
fn main() {
    let x = 3; // 这通常会警告未使用的变量。
}

//
// 属性的形式为 #[outer_attribute]（外部属性）或 #![inner_attribute]（内部属性），它们的区别在于应用的位置。
//
// #[outer_attribute] 应用于紧随其后的条目。
// 条目的例子包括：函数、模块声明、常量、结构体、枚举等。
// 以下是一个示例，其中属性 #[derive(Debug)] 应用于结构体 Rectangle：

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 属性可以使用不同的语法接受参数：

// #[attribute = "value"]
// #[attribute(key = "value")]
// #[attribute(value)]
// 属性可以有多个值，也可以跨多行分隔：
// #[attribute(value, value2)]
// #[attribute(value,
//     value2,
//     value3,
//     value4,
//     value5
// )]


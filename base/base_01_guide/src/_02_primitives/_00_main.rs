// 原生类型
// Rust 提供了多种原生类型。以下是一些示例：
//
// 标量类型
// 有符号整数：i8、i16、i32、i64、i128 和 isize（指针大小）
// 无符号整数：u8、u16、u32、u64、u128 和 usize（指针大小）
// 浮点数：f32、f64
// char Unicode 标量值，如 'a'、'α' 和 '∞'（每个都是 4 字节）
// bool 值为 true 或 false
// 单元类型 ()，其唯一可能的值是空元组：()
// 尽管单元类型的值是一个元组，但它不被视为复合类型，因为它不包含多个值。
//
// 复合类型
// 数组，如 [1, 2, 3]
// 元组，如 (1, true)
// 变量总是可以进行类型标注。数字还可以通过后缀或默认方式来标注。
// 整数默认为 i32 类型，浮点数默认为 f64 类型。请注意，Rust 也可以从上下文中推断类型。

#[wjj_lib::gen_test]
fn main() {
    // 变量可以被类型标注。
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 常规标注
    let an_integer   = 5i32; // 后缀标注

    // 或者使用默认类型。
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // 类型也可以从上下文中推断。
    let mut inferred_type = 12; // 从另一行推断出类型为 i64。
    inferred_type = 4294967296i64;

    // 可变变量的值可以改变。
    let mut mutable = 12; // 可变的 `i32`
    mutable = 21;

    // 报错！变量的类型不能改变。
    // mutable = true;

    // 变量可以通过遮蔽（shadowing）来覆盖。
    let mutable = true;

    /* 复合类型 - 数组和元组 */

    // 数组的签名由类型 T 和长度组成，表示为 [T; length]。
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // 元组是不同类型值的集合，
    // 使用圆括号 () 构造。
    let my_tuple = (5u32, 1u8, true, -5.04f32);
}


// 字面量
// 数字字面值可以通过添加类型后缀进行类型标注。例如，要指定字面值 42 的类型为 i32，可以写成 42i32。
//
// 无后缀数字字面值的类型取决于其使用方式。如果没有约束，编译器将对整数使用 i32，对浮点数使用 f64。

#[test]
fn main() {
    // 带后缀的字面值，其类型在初始化时确定
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面值，其类型取决于使用方式
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回变量的字节大小
    println!("`x` 的字节大小：{}", std::mem::size_of_val(&x));
    println!("`y` 的字节大小：{}", std::mem::size_of_val(&y));
    println!("`z` 的字节大小：{}", std::mem::size_of_val(&z));
    println!("`i` 的字节大小：{}", std::mem::size_of_val(&i));
    println!("`f` 的字节大小：{}", std::mem::size_of_val(&f));
}

// 前面的代码中使用了一些尚未解释的概念。为了满足迫不及待的读者，这里简要说明如下：
//
// std::mem::size_of_val 是一个函数，这里使用了它的"完整路径"来调用。
// 代码可以被划分为称为"模块"的逻辑单元。
// 在这个例子中，size_of_val 函数定义在 mem 模块中，而 mem 模块则定义在 std crate 中。
// 更多详情请参阅模块和crate。

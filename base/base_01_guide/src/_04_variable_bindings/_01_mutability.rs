// 可变性
// 变量绑定默认是不可变的，但可以使用 mut 修饰符来改变这一行为。

#[test]
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("修改前：{}", mutable_binding);

    // 正确
    mutable_binding += 1;

    println!("修改后：{}", mutable_binding);

    // 错误！不能给不可变变量赋新值
    // _immutable_binding += 1;
}

// 作为输入参数
// Rust 通常能自动选择如何捕获变量，无需类型标注。
// 但在编写函数时，这种模糊性是不允许的。
// 当将闭包作为输入参数时，必须使用特定的 trait 来注解闭包的完整类型。
// 这些 trait 由闭包对捕获值的处理方式决定。按限制程度从高到低排列如下：
//
// Fn：闭包通过引用使用捕获的值（&T）
// FnMut：闭包通过可变引用使用捕获的值（&mut T）
// FnOnce：闭包通过值使用捕获的值（T）
// 编译器会以尽可能最少限制的方式逐个捕获变量。
//
// 例如，考虑一个注解为 FnOnce 的参数。
// 这表示闭包可能通过 &T、&mut T 或 T 进行捕获，但编译器最终会根据捕获变量在闭包中的使用方式来决定。
//
// 这是因为如果可以移动，那么任何类型的借用也应该是可能的。
// 注意反过来并不成立。
// 如果参数被注解为 Fn，那么通过 &mut T 或 T 捕获变量是不允许的。但 &T 是允许的。
//
// 在下面的例子中，尝试交换 Fn、FnMut 和 FnOnce 的用法，看看会发生什么：

// 这个函数接受一个闭包作为参数并调用它
// <F> 表示 F 是一个"泛型类型参数"
fn apply<F>(f: F) where
// 这个闭包不接受输入也不返回任何值
    F: FnOnce() {
    // ^ TODO：试着将其改为 `Fn` 或 `FnMut`

    f();
}

// 这个函数接受一个闭包并返回 `i32`
fn apply_to_3<F>(f: F) -> i32 where
// 这个闭包接受一个 `i32` 并返回一个 `i32`
    F: Fn(i32) -> i32 {

    f(3)
}

#[wjj_lib::gen_test]
fn main() {
    use std::mem;

    let greeting = "hello";
    // 一个非复制类型
    // `to_owned` 从借用的数据创建拥有所有权的数据
    let mut farewell = "goodbye".to_owned();

    // 捕获两个变量：通过引用捕获 `greeting`，
    // 通过值捕获 `farewell`
    let diary = || {
        // `greeting` 是通过引用捕获的：需要 `Fn`
        println!("我说{}。", greeting);

        // 修改强制 `farewell` 通过可变引用捕获
        // 现在需要 `FnMut`
        farewell.push_str("！！！");
        println!("然后我喊{}。", farewell);
        println!("现在我可以睡觉了。呼呼");

        // 手动调用 drop 强制 `farewell` 通过值捕获
        // 现在需要 `FnOnce`
        mem::drop(farewell);
    };

    // 调用应用闭包的函数
    apply(diary);

    // `double` 满足 `apply_to_3` 的 trait 约束
    let double = |x| 2 * x;

    println!("3 的两倍是：{}", apply_to_3(double));
}
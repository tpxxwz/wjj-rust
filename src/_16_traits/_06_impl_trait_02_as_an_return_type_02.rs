// 更重要的是，某些 Rust 类型无法直接写出。
// 例如，每个闭包都有自己的未命名具体类型。在 impl Trait 语法出现之前，你必须在堆上分配内存才能返回闭包。
// 但现在你可以完全静态地做到这一点，像这样：

// 返回一个将 `y` 加到输入值上的函数
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}

crate::gen_test!(main);
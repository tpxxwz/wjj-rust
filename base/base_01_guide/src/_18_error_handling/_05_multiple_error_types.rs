// 多种错误类型
// 前面的例子一直都很方便：Result 与 Result 交互，Option 与 Option 交互。
//
// 有时，Option 需要与 Result 交互，或者 Result<T, Error1> 需要与 Result<T, Error2> 交互。
// 在这些情况下，我们希望以一种使不同错误类型可组合且易于交互的方式来管理它们。
//
// 在下面的代码中，两个 unwrap 实例生成了不同的错误类型。
// Vec::first 返回一个 Option，而 parse::<i32> 返回一个 Result<i32, ParseIntError>：

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // 生成错误 1
    2 * first.parse::<i32>().unwrap() // 生成错误 2
}

#[wjj_lib::gen_test]
fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("第一个数的两倍是 {}", double_first(numbers));

    println!("第一个数的两倍是 {}", double_first(empty));
    // 错误 1：输入向量为空

    println!("第一个数的两倍是 {}", double_first(strings));
    // 错误 2：元素无法解析为数字
}
// 在接下来的章节中，我们将探讨几种处理此类问题的策略。


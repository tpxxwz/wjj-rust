// 从 Option 中提取 Result
// 处理混合错误类型最基本的方法是将它们相互嵌套。

use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("第一个数的两倍是 {:?}", double_first(numbers));

    println!("第一个数的两倍是 {:?}", double_first(empty));
    // 错误 1：输入向量为空

    println!("第一个数的两倍是 {:?}", double_first(strings));
    // 错误 2：元素无法解析为数字
}

crate::gen_test!(main);
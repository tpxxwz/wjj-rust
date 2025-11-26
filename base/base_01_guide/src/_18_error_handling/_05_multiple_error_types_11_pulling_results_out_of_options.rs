// 从 Option 中提取 Result
// 处理混合错误类型最基本的方法是将它们相互嵌套。

use std::num::ParseIntError;

fn double_first1(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

#[wjj_lib::gen_test]
fn main1() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("第一个数的两倍是 {:?}", double_first1(numbers));

    println!("第一个数的两倍是 {:?}", double_first1(empty));
    // 错误 1：输入向量为空

    println!("第一个数的两倍是 {:?}", double_first1(strings));
    // 错误 2：元素无法解析为数字
}

// 有时我们希望在遇到错误时停止处理（例如使用 ?），但在 Option 为 None 时继续执行。
// 这时 transpose 函数就派上用场了，它可以方便地交换 Result 和 Option。

fn double_first2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.transpose()
}

#[wjj_lib::gen_test]
fn main2() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("第一个数的两倍是 {:?}", double_first2(numbers));
    println!("第一个数的两倍是 {:?}", double_first2(empty));
    println!("第一个数的两倍是 {:?}", double_first2(strings));
}


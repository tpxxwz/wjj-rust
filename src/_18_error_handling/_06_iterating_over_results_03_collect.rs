// 使用 collect() 使整个操作失败
// Result 实现了 FromIterator trait，因此结果的向量（Vec<Result<T, E>>）
// 可以转换为包含向量的结果（Result<Vec<T>, E>）。一旦遇到 Result::Err，迭代就会终止。

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("结果：{:?}", numbers);
}

crate::gen_test!(main);
// 遍历 Result
// Iter::map 操作可能会失败，例如：
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("结果：{:?}", numbers);
}
// 让我们逐步介绍处理这种情况的策略。

crate::gen_test!(main);
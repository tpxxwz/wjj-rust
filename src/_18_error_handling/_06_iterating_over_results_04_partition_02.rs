// 当你查看结果时，你会注意到所有内容仍然被包装在 Result 中。这需要一些额外的样板代码。

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("数字：{:?}", numbers);
    println!("错误：{:?}", errors);
}

crate::gen_test!(main);
// 使用 partition() 收集所有有效值和失败项

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("数字：{:?}", numbers);
    println!("错误：{:?}", errors);
}

crate::gen_test!(main);
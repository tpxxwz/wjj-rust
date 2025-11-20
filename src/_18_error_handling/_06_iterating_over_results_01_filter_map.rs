// 使用 filter_map() 忽略失败的项
// filter_map 调用一个函数并过滤掉结果为 None 的项。

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("结果：{:?}", numbers);
}

crate::gen_test!(main);
// 使用 map_err() 和 filter_map() 收集失败的项
// map_err 会对错误调用一个函数，因此将其添加到之前的 filter_map 解决方案中，我们可以在迭代时将错误项保存到一旁。

fn main() {
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("数字：{:?}", numbers);
    println!("错误：{:?}", errors);
}

crate::gen_test!(main);
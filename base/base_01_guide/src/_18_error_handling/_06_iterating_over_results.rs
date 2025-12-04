// 遍历 Result
// Iter::map 操作可能会失败，例如：
#[test]
fn main1() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("结果：{:?}", numbers);
}
// 让我们逐步介绍处理这种情况的策略。

// 使用 filter_map() 忽略失败的项
// filter_map 调用一个函数并过滤掉结果为 None 的项。
#[test]
fn main2() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("结果：{:?}", numbers);
}

// 使用 map_err() 和 filter_map() 收集失败的项
// map_err 会对错误调用一个函数，因此将其添加到之前的 filter_map 解决方案中，我们可以在迭代时将错误项保存到一旁。
#[test]
fn main3() {
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

// 使用 collect() 使整个操作失败
// Result 实现了 FromIterator trait，因此结果的向量（Vec<Result<T, E>>）
// 可以转换为包含向量的结果（Result<Vec<T>, E>）。一旦遇到 Result::Err，迭代就会终止。
#[test]
fn main4() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("结果：{:?}", numbers);
}

// 使用 partition() 收集所有有效值和失败项
#[test]
fn main51() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("数字：{:?}", numbers);
    println!("错误：{:?}", errors);
}

// 当你查看结果时，你会注意到所有内容仍然被包装在 Result 中。这需要一些额外的样板代码。
#[test]
fn main52() {
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

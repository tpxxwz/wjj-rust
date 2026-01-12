// while let
// 与 if let 类似，while let 可以简化繁琐的 match 序列。让我们来看一个递增 i 的例子：
#[test]
fn main1() {
    // 创建 `Option<i32>` 类型的 `optional`
    let mut optional = Some(0);

    // 重复执行此测试。
    loop {
        match optional {
            // 如果 `optional` 解构成功，则执行代码块。
            Some(i) => {
                if i > 9 {
                    println!("大于 9，退出！");
                    optional = None;
                } else {
                    println!("`i` 是 `{:?}`。再试一次。", i);
                    optional = Some(i + 1);
                }
                // ^ 需要 3 层缩进！
            }
            // 当解构失败时退出循环：
            _ => {
                break;
            } // ^ 为什么需要这样？一定有更好的方法！
        }
    }
}

// 使用 while let 可以让这个序列更加简洁：
#[test]
fn main2() {
    // 创建 `Option<i32>` 类型的 `optional`
    let mut optional = Some(0);

    // 这段代码的含义是：当 `let` 将 `optional` 解构为 `Some(i)` 时，
    // 执行代码块 `{}`，否则 `break`。
    while let Some(i) = optional {
        if i > 9 {
            println!("大于 9，退出！");
            optional = None;
        } else {
            println!("`i` 是 `{:?}`。再试一次。", i);
            optional = Some(i + 1);
        }
        // ^ 减少了代码缩进右移，无需显式处理失败情况
    }
    // ^ `if let` 可以有额外的 `else`/`else if` 子句，`while let` 则没有。
}

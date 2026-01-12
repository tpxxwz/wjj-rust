// match
// Rust 通过 match 关键字提供模式匹配，类似于 C 语言的 switch。
// 第一个匹配的分支会被求值，并且必须覆盖所有可能的值。

#[test]
fn main() {
    let number = 13;
    // TODO ^ 尝试为 `number` 赋不同的值

    println!("告诉我关于 {} 的信息", number);
    match number {
        // 匹配单个值
        1 => println!("一！"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("这是个质数"),
        // TODO ^ 尝试将 13 添加到质数列表中
        // 匹配一个闭区间范围
        13..=19 => println!("一个青少年"),
        // 处理其余情况
        _ => println!("没什么特别的"),
        // TODO ^ 尝试注释掉这个匹配所有情况的分支
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 的分支必须覆盖所有可能的值
        false => 0,
        true => 1,
        // TODO ^ 尝试注释掉其中一个分支
    };

    println!("{} -> {}", boolean, binary);
}

// loop
// Rust 提供 loop 关键字来表示无限循环。
//
// break 语句可以随时退出循环，而 continue 语句可以跳过当前迭代的剩余部分并开始下一次迭代。

#[test]
fn main() {
    let mut count = 0u32;

    println!("让我们数到无穷大！");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过本次迭代的剩余部分
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("好了，够了");

            // 退出这个循环
            break;
        }
    }
}

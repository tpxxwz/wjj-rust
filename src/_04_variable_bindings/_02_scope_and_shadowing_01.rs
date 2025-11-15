// 作用域和遮蔽
// 变量绑定有作用域，它们被限制在一个代码块中生存。代码块是由花括号 {} 包围的一系列语句。

fn main() {
    // 这个绑定存在于 main 函数中
    let long_lived_binding = 1;

    // 这是一个代码块，它的作用域比 main 函数小
    {
        // 这个绑定只存在于此代码块中
        let short_lived_binding = 2;

        println!("内部 short：{}", short_lived_binding);
    }
    // 代码块结束

    // 错误！`short_lived_binding` 在此作用域中不存在
    // println!("外部 short：{}", short_lived_binding);
    // 修复：^ 注释掉此行

    println!("外部 long：{}", long_lived_binding);
}
// 高阶函数
// Rust 提供了高阶函数（Higher Order Functions，HOF）。
// 这些函数接受一个或多个函数作为参数，并/或产生一个更有用的函数。
// HOF 和惰性迭代器赋予了 Rust 函数式编程的特性。

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

#[test]
fn main() {
    println!("找出所有平方为奇数且小于 1000 的数字之和");
    let upper = 1000;

    // 命令式方法
    // 声明累加器变量
    let mut acc = 0;
    // 迭代：从 0, 1, 2, ... 到无穷大
    for n in 0.. {
        // 计算数字的平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 如果超过上限则跳出循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数，则累加值
            acc += n_squared;
        }
    }
    println!("命令式风格：{}", acc);

    // 函数式方法
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // 所有自然数的平方
        .take_while(|&n_squared| n_squared < upper) // 小于上限
        .filter(|&n_squared| is_odd(n_squared)) // 筛选奇数
        .sum(); // 求和
    println!("函数式风格：{}", sum_of_squared_odd_numbers);
}
// Option 和 Iterator 实现了相当多的高阶函数。

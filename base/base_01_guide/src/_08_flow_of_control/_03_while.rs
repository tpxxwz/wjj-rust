// while
// while 关键字用于在条件为真时运行循环。
//
// 让我们用 while 循环来编写著名的 FizzBuzz 程序。

#[wjj_lib::gen_test]
fn main() {
    // 计数器变量
    let mut n = 1;

    // 当 `n` 小于 101 时继续循环
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器递增
        n += 1;
    }
}


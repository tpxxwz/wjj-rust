// for 循环
// for 和 range
// for in 结构可用于遍历 Iterator。创建迭代器最简单的方法之一是使用区间表示法 a..b。这会生成从 a（包含）到 b（不包含）的值，步长为 1。
//
// 让我们用 for 而不是 while 来编写 FizzBuzz。
fn main() {
    // `n` 在每次迭代中将取值：1, 2, ..., 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 另外，可以使用 a..=b 表示两端都包含的范围。上面的代码可以改写为：
    // `n` 在每次迭代中将取值：1, 2, ..., 100
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

crate::gen_test!(main);
// for 循环
// for 和 range
// for in 结构可用于遍历 Iterator。创建迭代器最简单的方法之一是使用区间表示法 a..b。这会生成从 a（包含）到 b（不包含）的值，步长为 1。
//
// 让我们用 for 而不是 while 来编写 FizzBuzz。
#[wjj_lib::gen_test]
fn main1() {
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

// for 与迭代器
// for in 结构能以多种方式与 Iterator 交互。
// 正如在 Iterator 特质一节中讨论的那样，默认情况下 for 循环会对集合应用 into_iter 函数。
// 然而，这并不是将集合转换为迭代器的唯一方法。
//
// into_iter、iter 和 iter_mut 都以不同的方式处理集合到迭代器的转换，通过提供对数据的不同视图。

#[wjj_lib::gen_test]
fn main2() {
    // iter - 在每次迭代中借用集合的每个元素。因此，集合保持不变，并且在循环之后可以重复使用。
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("我们中间有一个 Rustacean！"),
            // TODO ^ 尝试删除 & 并只匹配 "Ferris"
            _ => println!("你好 {}", name),
        }
    }

    println!("names: {:?}", names);

    // into_iter - 这会消耗集合，使得在每次迭代中提供确切的数据。
    // 一旦集合被消耗，它就不再可用于重复使用，因为它已经在循环中被"移动"了。
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("我们中间有一个 Rustacean！"),
            _ => println!("你好 {}", name),
        }
    }

    // println!("names: {:?}", names);
    // 修复：^ 注释掉此行

    // iter_mut - 这会可变地借用集合的每个元素，允许在原地修改集合。
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "我们中间有一个 Rustacean！",
            _ => "你好",
        }
    }

    println!("names: {:?}", names);
}
// 在上面的代码片段中，注意 match 分支的类型，这是迭代类型的关键区别。类型的差异意味着可以执行不同的操作。


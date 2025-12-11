// impl Trait
// impl Trait 可以在两个位置使用：
//
// 作为参数类型
// 作为返回类型

// 作为参数类型
// 如果你的函数对某个 trait 是泛型的，但不关心具体类型，你可以使用 impl Trait 作为参数类型来简化函数声明。
//
// 例如，考虑以下代码：

fn parse_csv_document1<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // 遍历数据源中的每一行
            line.map(|line| {
                // 如果成功读取行，则处理它；否则，返回错误
                line.split(',') // 按逗号分割行
                    .map(|entry| String::from(entry.trim())) // 去除首尾空白
                    .collect() // 将该行的所有字符串收集到 Vec<String> 中
            })
        })
        .collect() // 将所有行收集到 Vec<Vec<String>> 中
}

// parse_csv_document 是泛型函数，可以接受任何实现了 BufRead 的类型，如 BufReader<File> 或 [u8]。
// 但具体的 R 类型并不重要，R 仅用于声明 src 的类型。因此，这个函数也可以写成：

fn parse_csv_document2(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // 遍历数据源中的每一行
            line.map(|line| {
                // 如果成功读取行，则处理它；否则，返回错误
                line.split(',') // 按逗号分割行
                    .map(|entry| String::from(entry.trim())) // 去除首尾空白
                    .collect() // 将该行的所有字符串收集到 Vec<String> 中
            })
        })
        .collect() // 将所有行收集到 Vec<Vec<String>> 中
}
// 注意，使用 impl Trait 作为参数类型意味着你无法显式指定使用的函数形式。
// 例如，parse_csv_document::<std::io::Empty>(std::io::empty()) 在第二个例子中将无法工作。

// 作为返回类型
// 如果函数返回一个实现了 MyTrait 的类型，你可以将其返回类型写为 -> impl MyTrait。这可以大大简化类型签名！

use std::iter;
use std::vec::IntoIter;

// 这个函数合并两个 `Vec<i32>` 并返回一个迭代器。
// 看看它的返回类型有多复杂！
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 这是完全相同的函数，但它的返回类型使用了 `impl Trait`。
// 看看它变得多么简单！
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

#[test]
fn main1() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("全部完成");
}

// 更重要的是，某些 Rust 类型无法直接写出。
// 例如，每个闭包都有自己的未命名具体类型。在 impl Trait 语法出现之前，你必须在堆上分配内存才能返回闭包。
// 但现在你可以完全静态地做到这一点，像这样：

// 返回一个将 `y` 加到输入值上的函数
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

#[test]
fn main2() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}

// 你还可以使用 impl Trait 返回一个使用 map 或 filter 闭包的迭代器！这使得使用 map 和 filter 更加容易。
// 由于闭包类型没有名称，如果你的函数返回带有闭包的迭代器，你无法写出显式的返回类型。
// 但使用 impl Trait，你可以轻松实现：

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

#[test]
fn main3() {
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}

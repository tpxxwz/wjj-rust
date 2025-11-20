// 你还可以使用 impl Trait 返回一个使用 map 或 filter 闭包的迭代器！这使得使用 map 和 filter 更加容易。
// 由于闭包类型没有名称，如果你的函数返回带有闭包的迭代器，你无法写出显式的返回类型。
// 但使用 impl Trait，你可以轻松实现：

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn main() {
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}

crate::gen_test!(main);
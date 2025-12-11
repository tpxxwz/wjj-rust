// 通过迭代器搜索
// Iterator::find 是一个函数，它遍历迭代器并搜索满足特定条件的第一个值。
// 如果没有值满足条件，则返回 None。其签名如下：
pub trait Iterator {
    // 被迭代的类型
    type Item;

    // `find` 接受 `&mut self`，这意味着调用者可能被借用
    // 和修改，但不会被消耗。
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        // `FnMut` 表示任何捕获的变量最多只能被修改，不能被消耗。
        // `&Self::Item` 表示它通过引用将参数传递给闭包。
        P: FnMut(&Self::Item) -> bool;
}

#[test]
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 使用 `iter()` 会产生 `&i32`。
    let mut iter = vec1.iter();
    // 对 vec 使用 `into_iter()` 会产生 `i32`。
    let mut into_iter = vec2.into_iter();

    // 对 vec 使用 `iter()` 会产生 `&i32`，而我们想要引用其中的一个
    // 元素，所以我们必须将 `&&i32` 解构为 `i32`
    println!("在 vec1 中查找 2：{:?}", iter.find(|&&x| x == 2));
    // 对 vec 使用 `into_iter()` 会产生 `i32`，而我们想要引用其中的
    // 一个元素，所以我们必须将 `&i32` 解构为 `i32`
    println!("在 vec2 中查找 2：{:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组使用 `iter()` 会产生 `&&i32`
    println!("在 array1 中查找 2：{:?}", array1.iter().find(|&&x| x == 2));
    // 对数组使用 `into_iter()` 会产生 `&i32`
    println!(
        "在 array2 中查找 2：{:?}",
        array2.into_iter().find(|&x| x == 2)
    );

    // Iterator::find 返回元素的引用。如果需获取元素的索引，则使用 Iterator::position。
    let vec = vec![1, 9, 3, 3, 13, 2];

    // 对 vec 使用 `iter()` 会产生 `&i32`，而 `position()` 不接受引用，所以
    // 我们必须将 `&i32` 解构为 `i32`
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    // 对 vec 使用 `into_iter()` 会产生 `i32`，而 `position()` 不接受引用，所以
    // 我们不需要进行解构
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}

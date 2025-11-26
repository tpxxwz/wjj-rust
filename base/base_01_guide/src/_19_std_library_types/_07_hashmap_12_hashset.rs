// HashSet
// 可以将 HashSet 视为一个只关心键的 HashMap（实际上，HashSet<T> 只是 HashMap<T, ()> 的封装）。
//
// 你可能会问："这有什么意义？我可以直接把键存储在 Vec 中啊。"
//
// HashSet 的独特之处在于它保证不会有重复元素。这是所有集合类型都应满足的约定。
// HashSet 只是其中一种实现。（另请参阅：BTreeSet）
//
// 如果你插入一个已存在于 HashSet 中的值（即新值等于现有值，且它们的哈希值相同），那么新值将替换旧值。
//
// 当你不希望某个元素出现多次，或者想知道是否已经拥有某个元素时，这非常有用。
//
// 但是集合的功能不仅限于此。
//
// 集合有 4 种主要操作（以下所有调用都返回一个迭代器）：
//
// 并集（union）：获取两个集合中的所有唯一元素。
//
// 差集（difference）：获取存在于第一个集合但不在第二个集合中的所有元素。
//
// 交集（intersection）：获取同时存在于两个集合中的所有元素。
//
// 对称差（symmetric_difference）：获取存在于其中一个集合中，但不同时存在于两个集合中的所有元素。
//
// 在下面的例子中尝试这些操作：

use std::collections::HashSet;

#[wjj_lib::gen_test]
fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // 如果集合中已存在该值，`HashSet::insert()` 将返回 false。
    // assert!(b.insert(4), "值 4 已存在于集合 B 中！");
    // 修复：^ 注释掉此行

    b.insert(5);

    // 如果集合的元素类型实现了 `Debug` 特征，
    // 那么该集合也会实现 `Debug` 特征。
    // 通常会以 `[elem1, elem2, ...]` 的格式打印其元素
    println!("A：{:?}", a);
    println!("B：{:?}", b);

    // 以任意顺序打印 [1, 2, 3, 4, 5]
    println!("并集：{:?}", a.union(&b).collect::<Vec<&i32>>());

    // 这里应该打印 [1]
    println!("差集：{:?}", a.difference(&b).collect::<Vec<&i32>>());

    // 以任意顺序打印 [2, 3, 4]
    println!("交集：{:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // 打印 [1, 5]
    println!("对称差：{:?}",
             a.symmetric_difference(&b).collect::<Vec<&i32>>());
}


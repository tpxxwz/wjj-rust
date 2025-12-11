// 克隆
// 在处理资源时，默认行为是在赋值或函数调用期间转移它们。然而，有时我们也需要复制资源。
//
// Clone trait 帮助我们实现这一点。最常见的是，我们可以使用 Clone trait 定义的 .clone() 方法。

// 一个不包含资源的单元结构体
#[derive(Debug, Clone, Copy)]
struct Unit;

// 一个包含资源并实现了 `Clone` trait 的元组结构体
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

#[test]
fn main() {
    // 实例化 `Unit`
    let unit = Unit;
    // 复制 `Unit`，没有资源需要移动
    let copied_unit = unit;

    // 两个 `Unit` 可以独立使用
    println!("原始: {:?}", unit);
    println!("复制: {:?}", copied_unit);

    // 实例化 `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("原始: {:?}", pair);

    // 将 `pair` 移动到 `moved_pair`，资源也随之移动
    let moved_pair = pair;
    println!("已移动: {:?}", moved_pair);

    // 错误！`pair` 已失去其资源
    //println!("原始: {:?}", pair);
    // TODO ^ 尝试取消注释此行

    // 将 `moved_pair` 克隆到 `cloned_pair`（包括资源）
    let cloned_pair = moved_pair.clone();
    // 使用 std::mem::drop 丢弃已移动的原始对
    drop(moved_pair);

    // 错误！`moved_pair` 已被丢弃
    //println!("已移动并丢弃: {:?}", moved_pair);
    // TODO ^ 尝试取消注释此行

    // .clone() 的结果仍然可以使用！
    println!("克隆: {:?}", cloned_pair);
}

// 捕获
// 闭包本质上很灵活，无需注解就能根据功能需求自动适应。
// 这使得捕获可以灵活地适应不同场景，有时移动，有时借用。
//
// 闭包可以通过以下方式捕获变量：
// 通过引用：&T
// 通过可变引用：&mut T
// 通过值：T

#[wjj_lib::gen_test]
fn main() {
    // 闭包优先通过引用捕获变量，仅在必要时才使用更底部的的捕获方式。
    use std::mem;

    let color = String::from("green");

    // 打印 `color` 的闭包，立即借用（`&`）`color` 并
    // 将借用和闭包存储在 `print` 变量中。借用状态
    // 将持续到 `print` 最后一次使用。
    //
    // `println!` 只需要不可变引用参数，所以
    // 不会施加更多限制。
    let print = || println!("`color`: {}", color);

    // 使用借用调用闭包。
    print();

    // `color` 可以再次被不可变借用，因为闭包只持有
    // `color` 的不可变引用。
    let _reborrow = &color;
    print();

    // `print` 最后一次使用后，允许移动或重新借用
    let _color_moved = color;


    let mut count = 0;
    // 增加 `count` 的闭包可以接受 `&mut count` 或 `count`，
    // 但 `&mut count` 限制更少，所以选择它。立即
    // 借用 `count`。
    //
    // `inc` 需要 `mut` 因为内部存储了 `&mut`。因此，
    // 调用闭包会修改 `count`，这需要 `mut`。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包。
    inc();

    // 闭包仍然可变借用 `count`，因为它稍后会被调用。
    // 尝试重新借用会导致错误。
    // let _reborrow = &count;
    // ^ TODO：尝试取消注释这行。
    inc();

    // 闭包不再需要借用 `&mut count`。因此，
    // 可以在没有错误的情况下重新借用
    let _count_reborrowed = &mut count;


    // 不可复制类型。
    let movable = Box::new(3);

    // `mem::drop` 需要 `T`，所以这里必须通过值获取。可复制类型
    // 会被复制到闭包中，原始值保持不变。
    // 不可复制类型必须移动，所以 `movable` 立即移动到闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了变量，所以只能调用一次。
    consume();
    // consume();
    // ^ TODO：尝试取消注释这行。


    // 在竖线前使用 move 强制闭包获取捕获变量的所有权：
    // `Vec` 是非复制语义。
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("vec 中有 {} 个元素", haystack.len());
    // ^ 取消上面这行的注释会导致编译时错误
    // 因为借用检查器不允许在变量被移动后重用。


    // 从闭包签名中移除 `move` 将导致闭包
    // 不可变借用 _haystack_ 变量，因此 _haystack_ 仍可用，
    // 取消注释上面的行不会导致错误。
}


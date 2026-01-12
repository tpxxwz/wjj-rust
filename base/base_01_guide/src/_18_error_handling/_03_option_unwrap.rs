// Option 和 unwrap
// 在上一个例子中，我们展示了如何主动引发程序失败。我们让程序在喝含糖柠檬水时触发 panic。
// 但如果我们期望得到某种饮料却没有收到呢？这种情况同样糟糕，所以需要处理！
//
// 我们可以像处理柠檬水那样对空字符串（""）进行测试。但既然我们使用的是 Rust，不如让编译器指出没有饮料的情况。
//
// std 库中的 Option<T> 枚举用于处理可能存在缺失的情况。它表现为两个"选项"之一：
//
// Some(T)：找到了一个 T 类型的元素
// None：没有找到元素
// 这些情况可以通过 match 显式处理，也可以用 unwrap 隐式处理。隐式处理要么返回内部元素，要么触发 panic。
//
// 注意，可以使用 expect 手动自定义 panic，但 unwrap 相比显式处理会产生一个不太有意义的输出。
// 在下面的例子中，显式处理产生了一个更可控的结果，同时保留了在需要时触发 panic 的选项。

// 成年人见多识广，可以很好地处理任何饮料。
// 所有饮料都使用 `match` 显式处理。
fn give_adult(drink: Option<&str>) {
    // 为每种情况指定一个处理方案。
    match drink {
        Some("柠檬水") => println!("呸！太甜了。"),
        Some(inner) => println!("{}？真不错。", inner),
        None => println!("没有饮料？好吧。"),
    }
}

// 其他人在喝含糖饮料前会触发 `panic`。
// 所有饮料都使用 `unwrap` 隐式处理。
fn drink(drink: Option<&str>) {
    // 当 `unwrap` 收到 `None` 时会触发 `panic`。
    let inside = drink.unwrap();
    if inside == "柠檬水" {
        panic!("啊啊啊啊啊！！！！");
    }

    println!("我超爱{}！！！！！", inside);
}

#[test]
fn main() {
    let water = Some("水");
    let lemonade = Some("柠檬水");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("咖啡");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}

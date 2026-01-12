// 组合器：map
// match 是处理 Option 的有效方法。然而，频繁使用可能会让人感到繁琐，尤其是在只有输入时才有效的操作中。
// 在这些情况下，可以使用组合器以模块化的方式管理控制流。
//
// Option 有一个内置方法 map()，这是一个用于简单映射 Some -> Some 和 None -> None 的组合器。
// 多个 map() 调用可以链式使用，从而提供更大的灵活性。
//
// 在下面的例子中，process() 函数以简洁的方式替代了之前的所有函数。

#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

// 剥皮食物。如果没有食物，则返回 `None`。
// 否则，返回剥皮后的食物。
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

// 切碎食物。如果没有食物，则返回 `None`。
// 否则，返回切碎后的食物。
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// 烹饪食物。这里我们展示了使用 `map()` 而非 `match` 来处理不同情况。
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// 一个按顺序剥皮、切碎和烹饪食物的函数。
// 我们通过链式调用多个 `map()` 来简化代码。
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// 在尝试吃之前，先检查是否有食物！
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("嗯，我喜欢 {:?}", food),
        None => println!("哎呀！这不能吃。"),
    }
}

#[test]
fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // 现在让我们试试看起来更简洁的 `process()` 函数。
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

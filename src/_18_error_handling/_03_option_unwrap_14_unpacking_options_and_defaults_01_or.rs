// or() 可链式调用，立即求值，保持空值不变
// or() 可以链式调用，并且会立即求值其参数，如下例所示。
// 注意，由于 or 的参数是立即求值的，传递给 or 的变量会被移动。

#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("第一个可用的水果：{:?}", first_available_fruit);
    // first_available_fruit: Some(Orange)

    // `or` 会移动其参数。
    // 在上面的例子中，`or(orange)` 返回了 `Some`，所以 `or(apple)` 没有被调用。
    // 但是名为 `apple` 的变量无论如何都被移动了，不能再使用。
    // println!("变量 apple 被移动了，所以这行不会编译：{:?}", apple);
    // TODO：取消上面这行的注释来查看编译器错误
}

crate::gen_test!(main);
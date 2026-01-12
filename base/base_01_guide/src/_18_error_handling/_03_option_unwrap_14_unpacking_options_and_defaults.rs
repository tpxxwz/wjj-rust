// 解包 Option 和设置默认值
// 有多种方法可以解包 Option 并在其为 None 时使用默认值。为了选择满足我们需求的方法，我们需要考虑以下几点：
//
// 我们需要立即求值还是惰性求值？
// 我们需要保持原始的空值不变，还是就地修改它？

// or() 可链式调用，立即求值，保持空值不变
// or() 可以链式调用，并且会立即求值其参数，如下例所示。
// 注意，由于 or 的参数是立即求值的，传递给 or 的变量会被移动。

#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

#[test]
fn main1() {
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

// or_else() 可以链式调用，惰性求值，保持空值不变
// 另一种选择是使用 or_else，它同样支持链式调用，并且采用惰性求值。以下是一个示例：

#[test]
fn main2() {
    let no_fruit: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("提供猕猴桃作为备选");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("提供柠檬作为备选");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("第一个可用的水果：{:?}", first_available_fruit);
    // 提供猕猴桃作为备选
    // first_available_fruit: Some(Kiwi)
}

// get_or_insert() 立即求值，原地修改空值
// 为确保 Option 包含一个值，我们可以使用 get_or_insert 来原地修改它，提供一个备选值。
// 下面的例子展示了这一点。请注意，get_or_insert 会立即求值其参数，因此变量 apple 会被移动：

#[test]
fn main3() {
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("第一个可用的水果是：{:?}", first_available_fruit);
    println!("我的水果是：{:?}", my_fruit);
    // first_available_fruit is: Apple
    // my_fruit is: Some(Apple)
    //println!("名为 `apple` 的变量已被移动：{:?}", apple);
    // TODO：取消上面这行的注释以查看编译器错误
}

// get_or_insert_with() 惰性求值，原地修改空值
// 我们可以向 get_or_insert_with 传递一个闭包，而不是显式提供一个备选值。示例如下：

#[test]
fn main4() {
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("提供柠檬作为备选");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
    println!("第一个可用的水果是：{:?}", first_available_fruit);
    println!("我的水果是：{:?}", my_fruit);
    // 提供柠檬作为备选
    // first_available_fruit is: Lemon
    // my_fruit is: Some(Lemon)

    // 如果 Option 已有值，它将保持不变，闭包不会被调用
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple 的值为：{:?}", should_be_apple);
    println!("my_apple 保持不变：{:?}", my_apple);
    // 输出如下。注意闭包 `get_lemon_as_fallback` 并未被调用
    // should_be_apple is: Apple
    // my_apple is unchanged: Some(Apple)
}

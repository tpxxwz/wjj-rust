// get_or_insert_with() 惰性求值，原地修改空值
// 我们可以向 get_or_insert_with 传递一个闭包，而不是显式提供一个备选值。示例如下：

#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("提供柠檬作为备选");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit
        .get_or_insert_with(get_lemon_as_fallback);
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

crate::gen_test!(main);
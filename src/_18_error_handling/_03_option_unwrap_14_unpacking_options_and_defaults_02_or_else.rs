// or_else() 可以链式调用，惰性求值，保持空值不变
// 另一种选择是使用 or_else，它同样支持链式调用，并且采用惰性求值。以下是一个示例：

#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
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

crate::gen_test!(main);
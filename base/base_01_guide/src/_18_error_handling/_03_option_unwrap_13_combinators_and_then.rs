// 组合器：and_then
// map() 被描述为一种可链式调用的方式来简化 match 语句。
// 然而，在返回 Option<T> 的函数上使用 map() 会导致嵌套的 Option<Option<T>>。
// 链式调用多个这样的函数可能会变得令人困惑。
// 这时，另一个称为 and_then() 的组合器（在某些语言中称为 flatmap）就派上用场了。
//
// and_then() 使用包装的值调用其函数输入并返回结果。如果 Option 是 None，则直接返回 None。
//
// 在下面的例子中，cookable_v3() 返回一个 Option<Food>。
// 如果使用 map() 而不是 and_then()，将会得到一个 Option<Option<Food>>，这对于 eat() 函数来说是一个无效的类型。

#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    CordonBleu,
    Steak,
    Sushi,
}
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

// 我们没有制作寿司的原料。
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

// 我们有除了蓝带猪排以外所有菜品的食谱。
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

// 要做一道菜，我们需要食谱和原料。
// 我们可以用一系列的 `match` 来表示这个逻辑：
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => have_ingredients(food),
    }
}

// 这可以使用 `and_then()` 更简洁地重写：
fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

// 否则我们需要对 `Option<Option<Food>>` 使用 `flatten()`
// 来获得 `Option<Food>`：
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingredients).flatten()
}

fn eat(food: Food, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("太好了！在 {:?} 我们可以吃到 {:?}。", day, food),
        None => println!("哦不。我们在 {:?} 没有东西吃吗？", day),
    }
}

#[test]
fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}

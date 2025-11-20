// get_or_insert() 立即求值，原地修改空值
// 为确保 Option 包含一个值，我们可以使用 get_or_insert 来原地修改它，提供一个备选值。
// 下面的例子展示了这一点。请注意，get_or_insert 会立即求值其参数，因此变量 apple 会被移动：

#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
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

crate::gen_test!(main);
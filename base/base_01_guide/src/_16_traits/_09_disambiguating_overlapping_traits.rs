// 消除重叠特质的歧义
// 一个类型可以实现多个不同的 trait。如果两个 trait 都要求函数使用相同的名称，该怎么办？
// 例如，许多 trait 可能都有一个名为 get() 的方法，它们甚至可能有不同的返回类型！
//
// 好消息是：由于每个 trait 实现都有自己的 impl 块，因此很容易分清楚你正在实现哪个 trait 的 get 方法。
//
// 那么在调用这些方法时又该如何处理呢？为了消除它们之间的歧义，我们必须使用完全限定语法。

trait UsernameWidget {
    // 从这个小部件中获取选定的用户名
    fn get(&self) -> String;
}

trait AgeWidget {
    // 从这个小部件中获取选定的年龄
    fn get(&self) -> u8;
}

// 一个同时包含 UsernameWidget 和 AgeWidget 的表单
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

#[wjj_lib::gen_test]
fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // 如果你取消注释这一行，你会得到一个错误，提示
    // "找到多个 `get`"。这是因为确实存在多个
    // 名为 `get` 的方法。
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}


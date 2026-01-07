// 部分移动
// 在对单个变量进行解构时，可以同时使用 by-move 和 by-reference 模式绑定。
// 这样做会导致变量的部分移动，这意味着变量的一部分会被移动，而其他部分保持不变。
// 在这种情况下，父级变量之后不能作为一个整体使用，但是仍然可以使用只被引用（而不是被移动）的部分。
// 注意，实现了 Drop 特质 的类型不能被部分移动，因为其 drop 方法会在之后将其作为整体使用。

#[test]
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    // 错误！无法从实现了 `Drop` 特质的类型中移动
    //impl Drop for Person {
    //    fn drop(&mut self) {
    //        println!("Dropping the person struct {:?}", self)
    //    }
    //}
    // TODO ^ 尝试取消注释这些行

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` 从 person 中被移出，但 `age` 只是被引用
    let Person { name, ref age } = person;

    println!("此人的年龄是 {}", age);

    println!("此人的姓名是 {}", name);

    // 错误！借用部分移动的值：`person` 发生部分移动
    //println!("person 结构体是 {:?}", person);

    // `person` 不能使用，但 `person.age` 可以使用，因为它没有被移动
    println!("从 person 结构体中获取的年龄是 {}", person.age);
}
// （在这个例子中，我们将 age 变量存储在堆上以说明部分移动：
// 删除上面代码中的 ref 会导致错误，因为 person.age 的所有权会被移动到变量 age。
// 如果 Person.age 存储在栈上，就不需要 ref，因为 age 的定义会从 person.age 复制数据而不是移动它。）

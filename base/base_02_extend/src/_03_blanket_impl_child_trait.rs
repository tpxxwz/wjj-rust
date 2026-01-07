trait Parent {
    fn do_something(&self);
}
trait Child: Parent {
    // default implementation
    fn extra_method(&self) {
        println!("Default method in Child");
    }
}
struct MyStruct;
impl Parent for MyStruct {
    fn do_something(&self) {
        println!("Doing something");
    }
}
//  blanket implementation 泛型实现 用一个 impl 块为一大类类型提供 trait 实现
impl<T: Parent> Child for T {} // 能够自动让所有实现Parent trait的struct实现Child trait 能调用其中的默认方法
// impl Child for MyStruct {} // 或者加上这行手动实现Child trait，也能调用 extra_method()
#[test]
fn blanket_implementation() {
    let obj = MyStruct;
    obj.do_something();
    // 没有上面两种 MyStruct实现子trait的代码 这里会编译错误 ❌
    obj.extra_method();
}

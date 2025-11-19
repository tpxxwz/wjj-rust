// 析构函数
// Rust 中的析构函数概念是通过 Drop trait 提供的。
// 当资源离开作用域时，析构函数会被调用。
// 并非每种类型都需要实现这个 trait，只有当你需要为自己的类型实现特定的析构逻辑时才需要实现它。
//
// 运行下面的示例来了解 Drop trait 是如何工作的。
// 当 main 函数中的变量离开作用域时，自定义的析构函数将被调用。

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop 正在被丢弃");
    }
}

fn main() {
    let x = ToDrop;
    println!("创建了一个 ToDrop！");
}


crate::gen_test!(main);
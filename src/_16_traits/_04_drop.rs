// Drop
// Drop trait 只有一个方法：drop，它会在对象离开作用域时自动调用。
// Drop trait 的主要用途是释放实现该 trait 的实例所拥有的资源。
//
// Box、Vec、String、File 和 Process 是一些实现了 Drop trait 以释放资源的类型示例。
// 你也可以为任何自定义数据类型手动实现 Drop trait。
//
// 下面的例子在 drop 函数中添加了一个控制台打印，用于宣告它被调用的时机。

struct Droppable {
    name: &'static str,
}

// 这个简单的 `drop` 实现添加了一个控制台打印
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> 正在释放 {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // 块 A
    {
        let _b = Droppable { name: "b" };

        // 块 B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("正在退出块 B");
        }
        println!("刚刚退出了块 B");

        println!("正在退出块 A");
    }
    println!("刚刚退出了块 A");

    // 可以使用 `drop` 函数手动释放变量
    drop(_a);
    // TODO ^ 试试注释掉这一行

    println!("main 函数结束");

    // `_a` 在这里**不会**被再次 `drop`，因为它已经
    // 被（手动）`drop` 过了
}

crate::gen_test!(main);
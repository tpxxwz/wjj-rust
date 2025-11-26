// Arc
// 当需要在线程间共享所有权时，可以使用 Arc（原子引用计数，Atomically Reference Counted）。
// 通过 Clone 实现，这个结构体可以为堆内存中值的位置创建一个引用指针，同时增加引用计数。
// 由于它在线程间共享所有权，当指向某个值的最后一个引用指针超出作用域时，该变量就会被释放。

use std::time::Duration;
use std::sync::Arc;
use std::thread;

#[wjj_lib::gen_test]
fn main() {
    // 在这个变量声明中指定了它的值。
    let apple = Arc::new("同一个苹果");

    for _ in 0..10 {
        // 这里没有指定值，因为它是指向堆内存中引用的指针。
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // 由于使用了 Arc，可以使用 Arc 变量指针所指向的值来生成线程。
            println!("{:?}", apple);
        });
    }

    // 确保所有 Arc 实例都从生成的线程中打印出来。
    thread::sleep(Duration::from_secs(1));
}


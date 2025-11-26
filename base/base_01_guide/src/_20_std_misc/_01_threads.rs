// 线程
// Rust 通过 spawn 函数提供了一种生成原生操作系统线程的机制，该函数的参数是一个移动闭包。

use std::thread;

const NTHREADS: u32 = 10;

// 这是主线程
#[wjj_lib::gen_test]
fn main() {
    // 创建一个向量来存储生成的子线程
    let mut children = vec![];

    for i in 0..NTHREADS {
        // 启动一个新线程
        children.push(thread::spawn(move || {
            println!("这是第 {} 号线程", i);
        }));
    }

    for child in children {
        // 等待线程完成，返回一个结果
        let _ = child.join();
    }
}
// 这些线程将由操作系统进行调度。


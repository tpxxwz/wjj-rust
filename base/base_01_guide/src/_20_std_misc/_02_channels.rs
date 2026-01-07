// 信道
// Rust 提供异步通道（channels）用于线程间通信。
// 通道允许信息在两个端点之间单向流动：发送端（Sender）和接收端（Receiver）。

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

static NTHREADS: i32 = 3;

#[test]
fn main() {
    // 通道有两个端点：`Sender<T>` 和 `Receiver<T>`，
    // 其中 `T` 是要传输的消息类型
    // （此处的类型注解是多余的）
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // 发送端可以被复制
        let thread_tx = tx.clone();

        // 每个线程将通过通道发送其 ID
        let child = thread::spawn(move || {
            // 线程获取 `thread_tx` 的所有权
            // 每个线程在通道中排队一条消息
            thread_tx.send(id).unwrap();

            // 发送是非阻塞操作，线程在发送消息后
            // 会立即继续执行
            println!("线程 {} 已完成", id);
        });

        children.push(child);
    }

    // 在这里收集所有消息
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // `recv` 方法从通道中获取一条消息
        // 如果没有可用消息，`recv` 将阻塞当前线程
        ids.push(rx.recv());
    }

    // 等待线程完成所有剩余工作
    for child in children {
        child.join().expect("糟糕！子线程发生了 panic");
    }

    // 显示消息发送的顺序
    println!("{:?}", ids);
}

// Rust 中的 Mutex 用法
// Rust 提供 std::sync::Mutex，常用于多线程共享数据。

// 关键点：
// Mutex::new(0)：创建互斥锁保护的数据。
// lock()：获取锁，返回 MutexGuard，作用域结束自动释放锁。
// Arc：原子引用计数，用于多线程共享所有权。

// 常见用法场景
// 计数器：多个线程同时更新一个数值。
// 共享缓存：多个线程读写同一个 HashMap。
// 资源管理：文件、数据库连接等。

// 注意事项
// 锁竞争：多个线程同时请求锁会阻塞。
// 死锁风险：避免多个锁交叉等待。
// 性能影响：频繁加锁解锁会降低性能，适合小粒度锁。

// 总结：
// Mutex = 互斥锁，用于多线程安全访问共享数据，Rust 中结合 Arc 使用最常见。

use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn mutex_test() {
    let counter = Arc::new(Mutex::new(0)); // 共享计数器
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap(); // 获取锁
            // num 是MutexGuard对象 是锁的持有者，允许访问数据 可以通过 Deref 访问和修改数据 自动释放锁（RAII）
            *num += 1; // 修改共享数据
            println!("Thread number is {}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("结果: {}", *counter.lock().unwrap());
}

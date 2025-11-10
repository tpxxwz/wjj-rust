use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Mutex 学习示例 ===\n");

    // 基础 Mutex 使用
    basic_mutex();
    
    // Mutex 与 Arc 结合使用
    mutex_with_arc();
    
    // 线程间数据共享
    thread_data_sharing();
    
    // 生产者-消费者模式
    producer_consumer();
    
    // 读写锁模式
    read_write_pattern();
    
    // 高级用法
    advanced_usage();
}

fn basic_mutex() {
    println!("=== 基础 Mutex 使用 ===");
    
    // 创建 Mutex
    let mutex = Mutex::new(5);
    
    // 锁定并修改数据
    {
        let mut num = mutex.lock().unwrap();
        *num += 10;
        println!("修改后的值: {}", num);
    } // 锁在这里自动释放
    
    // 再次锁定读取数据
    println!("当前值: {}", *mutex.lock().unwrap());
    
    // 尝试锁定
    match mutex.try_lock() {
        Ok(value) => println!("成功获取锁: {}", value),
        Err(e) => println!("获取锁失败: {:?}", e),
    }
    
    println!();
}

fn mutex_with_arc() {
    println!("=== Mutex 与 Arc 结合使用 ===");
    
    // 创建可共享的互斥数据
    let counter = Arc::new(Mutex::new(0));
    
    // 创建多个线程
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("线程 {} 增加了计数器，当前值: {}", i, num);
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终计数器值: {}", *counter.lock().unwrap());
    println!();
}

fn thread_data_sharing() {
    println!("=== 线程间数据共享 ===");
    
    // 共享数据结构
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    
    let data_clone1 = Arc::clone(&data);
    let data_clone2 = Arc::clone(&data);
    
    // 线程1：添加元素
    let handle1 = thread::spawn(move || {
        let mut vec = data_clone1.lock().unwrap();
        vec.push(4);
        println!("线程1添加了元素，当前向量: {:?}", vec);
    });
    
    // 线程2：修改元素
    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100)); // 等待线程1完成
        let mut vec = data_clone2.lock().unwrap();
        vec[0] += 10;
        println!("线程2修改了第一个元素，当前向量: {:?}", vec);
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("最终数据: {:?}", *data.lock().unwrap());
    println!();
}

fn producer_consumer() {
    println!("=== 生产者-消费者模式 ===");
    
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    
    // 共享队列
    let queue: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));
    
    // 创建通道用于同步
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    
    // 生产者线程
    let queue_producer = Arc::clone(&queue);
    let producer = thread::spawn(move || {
        for i in 0..5 {
            thread::sleep(Duration::from_millis(100));
            let mut q = queue_producer.lock().unwrap();
            q.push(i);
            println!("生产者生产了: {}", i);
            tx.send(i).unwrap();
        }
        println!("生产者完成生产");
    });
    
    // 消费者线程
    let queue_consumer = Arc::clone(&queue);
    let consumer = thread::spawn(move || {
        let mut consumed = 0;
        while consumed < 5 {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(item) => {
                    let mut q = queue_consumer.lock().unwrap();
                    if let Some(pos) = q.iter().position(|&x| x == item) {
                        q.remove(pos);
                        println!("消费者消费了: {}", item);
                        consumed += 1;
                    }
                }
                Err(_) => {
                    println!("消费者等待超时");
                    break;
                }
            }
        }
        println!("消费者完成消费");
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    
    println!("最终队列状态: {:?}", *queue.lock().unwrap());
    println!();
}

fn read_write_pattern() {
    println!("=== 读写锁模式 ===");
    
    // 模拟一个共享的数据库
    let database = Arc::new(Mutex::new(vec![
        (1, "Alice".to_string()),
        (2, "Bob".to_string()),
        (3, "Charlie".to_string()),
    ]));
    
    let mut handles = vec![];
    
    // 创建多个读者线程
    for i in 0..3 {
        let db = Arc::clone(&database);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 100));
            let data = db.lock().unwrap();
            println!("读者 {} 读取数据: {:?}", i, data);
        });
        handles.push(handle);
    }
    
    // 创建一个写者线程
    let db_writer = Arc::clone(&database);
    let writer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(150));
        let mut data = db_writer.lock().unwrap();
        data.push((4, "David".to_string()));
        println!("写者添加了新数据");
    });
    
    handles.push(writer);
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终数据库: {:?}", *database.lock().unwrap());
    println!();
}

fn advanced_usage() {
    println!("=== 高级用法 ===");
    
    // 1. 嵌套 Mutex
    let nested = Arc::new(Mutex::new(Mutex::new(42)));
    {
        let outer = nested.lock().unwrap();
        let mut inner = outer.lock().unwrap();
        *inner += 1;
        println!("嵌套 Mutex 值: {}", inner);
    }
    
    // 2. Mutex 守卫的生命周期
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);
    
    let handle = thread::spawn(move || {
        // 获取锁并立即解构守卫
        let value = {
            let guard = data_clone.lock().unwrap();
            guard.len() // 只获取长度，然后释放锁
        }; // 守卫在这里被释放，锁被释放
        
        println!("向量的长度: {}", value);
        
        // 可以再次获取锁
        let mut guard = data_clone.lock().unwrap();
        guard.push(4);
        println!("修改后的向量: {:?}", guard);
    });
    
    handle.join().unwrap();
    
    // 3. 使用 Mutex 实现线程安全的缓存
    thread_safe_cache();
    
    // 4. 条件变量模式（简化版）
    conditional_variable_pattern();
    
    println!();
}

fn thread_safe_cache() {
    println!("=== 线程安全的缓存 ===");
    
    use std::collections::HashMap;
    
    struct Cache {
        data: Mutex<HashMap<String, i32>>,
    }
    
    impl Cache {
        fn new() -> Self {
            Cache {
                data: Mutex::new(HashMap::new()),
            }
        }
        
        fn get(&self, key: &str) -> Option<i32> {
            let cache = self.data.lock().unwrap();
            cache.get(key).copied()
        }
        
        fn set(&self, key: String, value: i32) {
            let mut cache = self.data.lock().unwrap();
            cache.insert(key, value);
        }
        
        fn get_or_insert(&self, key: String, default: i32) -> i32 {
            let mut cache = self.data.lock().unwrap();
            *cache.entry(key.clone()).or_insert(default)
        }
    }
    
    let cache = Arc::new(Cache::new());
    let mut handles = vec![];
    
    // 多个线程同时访问缓存
    for i in 0..5 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let key = format!("key{}", i);
            let value = cache.get_or_insert(key.clone(), i * 10);
            println!("线程 {} 获取值: {} = {}", i, key, value);
            
            // 再次访问同一个key
            let value2 = cache.get(&key).unwrap();
            println!("线程 {} 再次获取: {} = {}", i, key, value2);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("缓存最终状态有 {} 个元素", cache.data.lock().unwrap().len());
}

fn conditional_variable_pattern() {
    println!("=== 条件变量模式（简化版） ===");
    
    let flag = Arc::new(Mutex::new(false));
    let flag_clone = Arc::clone(&flag);
    
    // 等待线程
    let waiter = thread::spawn(move || {
        println!("等待线程开始等待");
        loop {
            let mut f = flag_clone.lock().unwrap();
            if *f {
                println!("等待线程检测到条件满足");
                break;
            }
            // 释放锁并让出CPU
            drop(f);
            thread::sleep(Duration::from_millis(10));
        }
    });
    
    // 通知线程
    let notifier = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        println!("通知线程设置条件");
        let mut f = flag.lock().unwrap();
        *f = true;
    });
    
    waiter.join().unwrap();
    notifier.join().unwrap();
    
    println!("条件变量模式完成");
}
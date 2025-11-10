// 18_arc.rs

use std::sync::Arc;
use std::thread;
use std::sync::Mutex;
use std::time::Duration;

fn main() {
    println!("=== Arc<T> 原子引用计数智能指针学习示例 ===\n");
    
    // 1. 基础创建和使用
    basic_usage();
    
    // 2. 线程间共享数据
    sharing_between_threads();
    
    // 3. 引用计数操作
    reference_counting();
    
    // 4. Arc和并发数据结构
    concurrent_data_structures();
    
    // 5. 高级并发模式
    advanced_concurrency();
}

// 1. 基础创建和使用
fn basic_usage() {
    println!("1. Arc基础创建和使用:");
    
    // 创建Arc
    let arc1 = Arc::new(42);
    println!("   Arc::new(42): {:?}", arc1);
    println!("   Arc值: {}", *arc1);
    
    // 克隆Arc（原子增加引用计数）
    let arc2 = Arc::clone(&arc1);
    println!("   克隆后Arc值: {}", *arc2);
    println!("   arc1和arc2是否指向同一数据: {}", Arc::ptr_eq(&arc1, &arc2));
    
    // 使用Arc::from从Box创建
    let boxed = Box::new("Hello Arc");
    let arc_from_box = Arc::from(boxed);
    println!("   从Box创建: {}", arc_from_box);
    
    // 获取引用计数
    let arc3 = Arc::clone(&arc1);
    println!("   引用计数: {}", Arc::strong_count(&arc1));
    
    // Arc和Rc的区别
    println!("   Arc大小: {} 字节", std::mem::size_of::<Arc<i32>>());
    println!("   Rc大小: {} 字节", std::mem::size_of::<std::rc::Rc<i32>>());
    
    // Arc的线程安全性
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}
    
    // 这可以编译，因为Arc是Send + Sync
    is_send::<Arc<i32>>();
    is_sync::<Arc<i32>>();
    println!("   Arc是Send + Sync，可以在线程间安全共享");
    
    println!();
}

// 2. 线程间共享数据
fn sharing_between_threads() {
    println!("2. 线程间共享数据:");
    
    // 共享不可变数据
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let sum: i32 = data_clone.iter().sum();
            println!("   线程{}计算的和: {}", i, sum);
            sum
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let result = handle.join().unwrap();
        println!("   线程结果: {}", result);
    }
    
    // 共享可变数据（使用Mutex）
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("   线程{}增加后计数: {}", i, *num);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("   最终计数: {}", *counter.lock().unwrap());
    
    // 共享字符串数据
    let shared_string = Arc::new(String::from("Hello from threads!"));
    let mut handles = vec![];
    
    for i in 0..3 {
        let string_clone = Arc::clone(&shared_string);
        let handle = thread::spawn(move || {
            println!("   线程{}读取字符串: '{}'", i, *string_clone);
            string_clone.len()
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let len = handle.join().unwrap();
        println!("   字符串长度: {}", len);
    }
    
    println!();
}

// 3. 引用计数操作
fn reference_counting() {
    println!("3. 引用计数操作:");
    
    // 创建Arc并观察引用计数变化
    let data = Arc::new(1000);
    println!("   初始引用计数: {}", Arc::strong_count(&data));
    
    {
        let clone1 = Arc::clone(&data);
        println!("   克隆后引用计数: {}", Arc::strong_count(&data));
        
        {
            let clone2 = Arc::clone(&data);
            println!("   再次克隆引用计数: {}", Arc::strong_count(&data));
        }
        
        println!("   clone2离开作用域后引用计数: {}", Arc::strong_count(&data));
    }
    
    println!("   clone1离开作用域后引用计数: {}", Arc::strong_count(&data));
    
    // 线程中的引用计数
    let data = Arc::new(vec![1, 2, 3]);
    println!("   线程前引用计数: {}", Arc::strong_count(&data));
    
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        println!("   线程内引用计数: {}", Arc::strong_count(&data_clone));
        data_clone.len()
    });
    
    handle.join().unwrap();
    println!("   线程结束后引用计数: {}", Arc::strong_count(&data));
    
    // 弱引用
    use std::sync::Weak;
    
    let strong = Arc::new("Strong data");
    let weak = Arc::downgrade(&strong);
    
    println!("   强引用计数: {}", Arc::strong_count(&strong));
    println!("   弱引用计数: {}", Arc::weak_count(&strong));
    
    match weak.upgrade() {
        Some(arc) => println!("   弱引用升级成功: {}", arc),
        None => println!("   弱引用升级失败"),
    }
    
    println!();
}

// 4. Arc和并发数据结构
fn concurrent_data_structures() {
    println!("4. Arc和并发数据结构:");
    
    // 并发哈希表（简化版）
    use std::collections::HashMap;
    
    let shared_map = Arc::new(Mutex::HashMap::new());
    let mut handles = vec![];
    
    // 多个线程同时写入
    for i in 0..5 {
        let map_clone = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            let mut map = map_clone.lock().unwrap();
            map.insert(format!("key{}", i), i * 10);
            println!("   线程{}插入数据", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_map = shared_map.lock().unwrap();
    println!("   最终哈希表长度: {}", final_map.len());
    
    // 并发向量处理
    let data = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    
    // 分块处理数据
    let chunk_size = 3;
    for chunk in data.chunks(chunk_size) {
        let data_chunk = chunk.to_vec();
        let results_clone = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            let sum: i32 = data_chunk.iter().sum();
            results_clone.lock().unwrap().push(sum);
            println!("   处理块 {:?}，和: {}", data_chunk, sum);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_results = results.lock().unwrap();
    let total_sum: i32 = final_results.iter().sum();
    println!("   所有块的总和: {}", total_sum);
    
    // 并发缓存
    #[derive(Debug)]
    struct Cache {
        data: Mutex<HashMap<String, String>>,
    }
    
    impl Cache {
        fn new() -> Arc<Self> {
            Arc::new(Cache {
                data: Mutex::new(HashMap::new()),
            })
        }
        
        fn get(&self, key: &str) -> Option<String> {
            self.data.lock().unwrap().get(key).cloned()
        }
        
        fn set(&self, key: String, value: String) {
            self.data.lock().unwrap().insert(key, value);
        }
    }
    
    let cache = Cache::new();
    let mut handles = vec![];
    
    // 多个线程读写缓存
    for i in 0..3 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let key = format!("key{}", i);
            let value = format!("value{}", i * 100);
            cache_clone.set(key.clone(), value.clone());
            
            // 稍微延迟让其他线程有机会写入
            thread::sleep(Duration::from_millis(10));
            
            match cache_clone.get(&key) {
                Some(v) => println!("   线程{}读取到: {}", i, v),
                None => println!("   线程{}未读取到值", i),
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!();
}

// 5. 高级并发模式
fn advanced_concurrency() {
    println!("5. 高级并发模式:");
    
    // 生产者-消费者模式
    use std::sync::mpsc;
    
    let (tx, rx) = mpsc::channel();
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    
    // 生产者线程
    let data_clone = Arc::clone(&shared_data);
    let tx_clone = tx.clone();
    let producer = thread::spawn(move || {
        for i in 0..5 {
            let item = format!("Item{}", i);
            data_clone.lock().unwrap().push(item.clone());
            tx_clone.send(item).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // 消费者线程
    let consumer = thread::spawn(move || {
        let mut count = 0;
        for received in rx {
            println!("   消费者收到: {}", received);
            count += 1;
        }
        count
    });
    
    producer.join().unwrap();
    drop(tx); // 关闭发送端
    
    let consumed_count = consumer.join().unwrap();
    println!("   消费了{}个项目", consumed_count);
    
    // 读写锁模式
    use std::sync::RwLock;
    
    let shared_config = Arc::new(RwLock::new(String::from("Initial Config")));
    let mut handles = vec![];
    
    // 多个读线程
    for i in 0..3 {
        let config_clone = Arc::clone(&shared_config);
        let handle = thread::spawn(move || {
            let config = config_clone.read().unwrap();
            println!("   读线程{}读取配置: '{}'", i, *config);
            thread::sleep(Duration::from_millis(50));
        });
        handles.push(handle);
    }
    
    // 一个写线程
    let config_clone = Arc::clone(&shared_config);
    let writer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(25));
        let mut config = config_clone.write().unwrap();
        *config = String::from("Updated Config");
        println!("   写线程更新了配置");
    });
    
    for handle in handles {
        handle.join().unwrap();
    }
    writer.join().unwrap();
    
    let final_config = shared_config.read().unwrap();
    println!("   最终配置: '{}'", *final_config);
    
    // 线程池模式（简化版）
    struct ThreadPool {
        workers: Vec<thread::JoinHandle<()>>,
    }
    
    impl ThreadPool {
        fn new(size: usize, shared_data: Arc<Mutex<Vec<i32>>>) -> Self {
            let mut workers = Vec::with_capacity(size);
            
            for id in 0..size {
                let data_clone = Arc::clone(&shared_data);
                let worker = thread::spawn(move || {
                    let mut data = data_clone.lock().unwrap();
                    data.push(id * 10);
                    println!("   工作线程{}完成任务", id);
                });
                workers.push(worker);
            }
            
            ThreadPool { workers }
        }
        
        fn wait_for_completion(self) {
            for worker in self.workers {
                worker.join().unwrap();
            }
        }
    }
    
    let shared_work = Arc::new(Mutex::new(Vec::new()));
    let pool = ThreadPool::new(4, shared_work);
    
    pool.wait_for_completion();
    
    let final_work = shared_work.lock().unwrap();
    println!("   线程池完成的工作: {:?}", *final_work);
    
    println!();
}
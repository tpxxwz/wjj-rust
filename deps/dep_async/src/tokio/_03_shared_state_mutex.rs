use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::yield_now;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::test]
async fn shared_state_mutex() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // Clone the handle to the hash map.
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!(
                    "op set key:{}, value:{:?}",
                    cmd.key().to_string(),
                    cmd.value()
                );
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("op get key:{}", cmd.key().to_string());
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

#[tokio::test]
async fn increment_and_do_stuff_test() {
    tokio::spawn(async {
        // increment_and_do_stuff_can_work(&Mutex::new(5i32)).await // 会造成死锁
        // increment_and_do_stuff_cannot_work(&Mutex::new(5i32)).await // 不能编译成功
        // 优化包装 包装在 fn increment(&self) { 这个方法的作用域lock取MutexGuard就没关系
        // increment_and_do_stuff(&CanIncrement {
        //     mutex: Mutex::new(5i32),
        // })
        // .await;
        increment_and_do_stuff_tokio_mutex(&tokio::sync::Mutex::new(5i32)).await;
    });
}
async fn do_something_async() {
    println!("do_something_async");
}
// MutexGuard 而且没有跨越 await 所以没关系
async fn increment_and_do_stuff_can_work(mutex: &Mutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    } // lock goes out of scope here

    do_something_async().await;
} // lock goes out of scope here

// MutexGuard 并没有实现 trait Send 所以不能在spawn里使用
async fn increment_and_do_stuff_cannot_work(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    *lock += 1;
    drop(lock);

    do_something_async().await;
}

// 重构你的代码，使其不要跨线程持有锁。.await
// 处理互斥锁最安全的方法是将其包装在一个结构体中，并且仅在该结构体的非异步方法中锁定互斥锁。
struct CanIncrement {
    mutex: Mutex<i32>,
}
impl CanIncrement {
    // This function is not marked async.
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}

async fn increment_and_do_stuff(can_incr: &CanIncrement) {
    can_incr.increment();
    do_something_async().await;
}

// This compiles!
// (but restructuring the code would be better in this case)
// using &tokio::sync::Mutex
async fn increment_and_do_stuff_tokio_mutex(mutex: &tokio::sync::Mutex<i32>) {
    let mut lock = mutex.lock().await;
    *lock += 1;

    do_something_async().await;
    do_something_async().await;
} // lock goes out of scope here

// Mutex sharding  map 分片思想

type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

async fn process_share_db(socket: TcpStream, db: ShardedDb) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!(
                    "op set key:{}, value:{:?}",
                    cmd.key().to_string(),
                    cmd.value()
                );
                let shard_index = (hash(&cmd.key()) % db.len() as u64) as usize;
                let mut shard = (*db)[shard_index].lock().unwrap();
                shard.insert(cmd.key().to_string(), cmd.value().clone().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("op get key:{}", cmd.key().to_string());
                let shard_index = (hash(&cmd.key()) % db.len() as u64) as usize;
                let mut shard = (*db)[shard_index].lock().unwrap();
                if let Some(value) = shard.get(cmd.key()) {
                    Frame::Bulk(Bytes::from(value.clone()))
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

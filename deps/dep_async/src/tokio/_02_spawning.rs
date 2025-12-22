use mini_redis::{Connection, Frame};
use std::rc::Rc;
use tokio::net::{TcpListener, TcpStream};

#[tokio::test]
async fn mock_redis_server() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    // 一次只能处理一个请求 只能等一个请求完成之后才会进入到下一次循环接受下一个来的请求
    // loop {
    //     // The second item contains the IP and port of the new connection.
    //     let (socket, _) = listener.accept().await.unwrap();
    //     process(socket).await;
    // }

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            // process_1(socket).await;
            process_2(socket).await;
        });
    }
}

async fn process_1(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}

async fn process_2(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // A hashmap is used to store data
    let mut db = HashMap::new();

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
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

// --tokio Tasks
// A Tokio task is an asynchronous green thread.
// They are created by passing an async block to tokio::spawn.
// The tokio::spawn function returns a JoinHandle,
// which the caller may use to interact with the spawned task.
// The async block may have a return value.
// The caller may obtain the return value using .await on the JoinHandle.

#[tokio::test]
async fn tokio_task() {
    // returns a JoinHandle
    let mut handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    // Do some other work
    let out = handle.await.unwrap();
    println!("GOT {}", out);

    handle = tokio::spawn(async {
        panic!("Something went wrong!");
    });

    match handle.await {
        Ok(_) => println!("Task completed successfully"),
        Err(e) => println!("Task failed: {:?}", e), // 这里会捕获 panic
    }
}

// 'static bound
// 编译器要求：异步任务必须是 'static（即不依赖外部引用），否则可能悬空引用
use tokio::task;
use tokio::task::yield_now;

#[tokio::main]
async fn task_need_static_bound() {
    let v = vec![1, 2, 3];

    // task::spawn(async { // 不可以编译过 报错
    //     println!("Here's a vec: {:?}", v);
    // });

    task::spawn(async move {
        // move 关键字会 把 v 的所有权移动到闭包里。 所以可以编译过。
        println!("Here's a vec: {:?}", v);
    });
}

// Send bound
// spawn 里的Future 需要实现 trait Future Send  F: Future + Send + 'static,
// 默认future的闭包方法里面没有 Rc RefCell MutexGuard !Send的第三方类型 等的话 就自动推导实现了Send
#[tokio::test]
async fn send_bound_without_rc() {
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });
}

// 如果里面有Rc对象 则不行 不能被作为Send
// #[tokio::test]
// async fn send_bound_with_rc() {
//     tokio::spawn(async {
//         let rc = Rc::new("hello");
//
//         // `rc` is used after `.await`. It must be persisted to
//         // the task's state.
//         yield_now().await;
//
//         println!("{}", rc);
//     });
// }

use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{broadcast, mpsc, oneshot, watch};
use tokio::task;

// channel_mpsc 适用于多个任务发送消息到一个接收端。
#[tokio::test]
async fn channel_mpsc() {
    let (tx, mut rx) = mpsc::channel(32);

    for i in 0..5 {
        let tx_clone = tx.clone();
        task::spawn(async move {
            tx_clone.send(format!("消息 {}", i)).await.unwrap();
        });
    }

    drop(tx); // 关闭发送端，防止接收端无限等待

    while let Some(msg) = rx.recv().await {
        println!("收到: {}", msg);
    }
}

// channel_oneshot 适用于一次性发送一个值。
#[tokio::test]
async fn channel_oneshot() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send("Hello oneshot").unwrap();
    });

    match rx.await {
        Ok(msg) => println!("收到: {}", msg),
        Err(_) => println!("发送端已关闭"),
    }
}

// channel_broadcast 适用于发布订阅模式。
#[tokio::test]
async fn channel_broadcast() {
    let (tx, _) = broadcast::channel(16);

    for i in 0..3 {
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                println!("订阅者 {} 收到: {}", i, msg);
                // 默认的打印 rx.recv().await 取到的时候一下取了两个 广播消息 1 广播消息 2了 直接两个都处理了
                // 订阅者 0 收到: 广播消息 1
                // 订阅者 0 收到: 广播消息 2
                // 订阅者 1 收到: 广播消息 1
                // 订阅者 1 收到: 广播消息 2
                // 订阅者 2 收到: 广播消息 1
                // 订阅者 2 收到: 广播消息 2

                tokio::task::yield_now().await // 主动让出执行权
                // 打印的结果就是
                // 订阅者 0 收到: 广播消息 1
                // 订阅者 1 收到: 广播消息 1
                // 订阅者 2 收到: 广播消息 1
                // 订阅者 2 收到: 广播消息 2
                // 订阅者 1 收到: 广播消息 2
                // 订阅者 0 收到: 广播消息 2
            }
        });
    }
    tx.send("广播消息 1").unwrap();
    tx.send("广播消息 2").unwrap();
    // 等待一段时间让消费者打印
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}

// channel_watch 适用于状态共享，接收者只关心最新值。
#[tokio::test]
async fn channel_watch() {
    let (tx, rx) = watch::channel(String::from("初始值"));

    // 启动多个消费者
    for id in 1..=3 {
        let mut rx_clone = rx.clone(); // 可以直接 rx.clone()，因为它是多消费者设计 其他的不行
        task::spawn(async move {
            while rx_clone.changed().await.is_ok() {
                println!("消费者 {} 收到最新值: {}", id, *rx_clone.borrow());
            }
        });
    }

    // 生产者更新状态
    tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(format!("更新值 {}", i)).unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    // 等待一段时间让消费者打印
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::test]
async fn channel_learn() {
    let (tx, mut rx) = mpsc::channel(32);

    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        // Send the GET request
        tx.send(cmd).await.unwrap();

        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        // Send the SET request
        tx2.send(cmd).await.unwrap();

        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
        // The `move` keyword is used to **move** ownership of `rx` into the task.
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

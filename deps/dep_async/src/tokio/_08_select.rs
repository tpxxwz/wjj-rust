use std::net::SocketAddr;
use std::pin::Pin;
use std::task::Poll::Pending;
use std::task::{Context, Poll};
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot::{Receiver, Sender};
use tokio::sync::{mpsc, oneshot};

// tokio::spawn 是真的操作系统的不同线程上做并发
// select! 是一个线程在做轮询不同分支 直到取到一个完成的

async fn select_demo() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let result = tx1.send("one");
        match result {
            Ok(str) => println!("tx1 send success {:?}", str),
            Err(e) => println!("tx1 send failed {}", e),
        }
    });

    tokio::spawn(async {
        let result = tx2.send("two");
        match result {
            Ok(str) => println!("tx2 send success {:?}", str),
            Err(e) => println!("tx2 send failed {}", e),
        }
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}

#[tokio::test]
async fn select_demo_test() {
    select_demo().await;
    // select_demo_expand().await;
}

async fn select_demo_expand() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    tokio::spawn(async {
        let result = tx1.send("one");
        match result {
            Ok(str) => println!("tx1 send success {:?}", str),
            Err(e) => println!("tx1 send failed {}", e),
        }
    });
    tokio::spawn(async {
        let result = tx2.send("two");
        match result {
            Ok(str) => println!("tx2 send success {:?}", str),
            Err(e) => println!("tx2 send failed {}", e),
        }
    });
    {
        #[doc(hidden)]
        mod __tokio_select_util {
            pub(super) enum Out<_0, _1> {
                _0(_0),
                _1(_1),
                Disabled,
            }
            pub(super) type Mask = u8;
        }
        use ::tokio::macros::support::Future;
        use ::tokio::macros::support::Pin;
        use ::tokio::macros::support::Poll::{Pending, Ready};
        const BRANCHES: u32 = 2;
        let mut disabled: __tokio_select_util::Mask = Default::default();
        if !true {
            let mask: __tokio_select_util::Mask = 1 << 0;
            disabled |= mask;
        }
        if !true {
            let mask: __tokio_select_util::Mask = 1 << 1;
            disabled |= mask;
        }
        let mut output = {
            let futures_init = (rx1, rx2);
            let mut futures = (
                ::tokio::macros::support::IntoFuture::into_future(futures_init.0),
                ::tokio::macros::support::IntoFuture::into_future(futures_init.1),
            );
            let mut futures = &mut futures;
            ::tokio::macros::support::poll_fn(|cx| {
                match ::tokio::macros::support::poll_budget_available(cx) {
                    ::core::task::Poll::Ready(t) => t,
                    ::core::task::Poll::Pending => {
                        return ::core::task::Poll::Pending;
                    }
                };
                let mut is_pending = false;
                let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
                for i in 0..BRANCHES {
                    let branch;
                    #[allow(clippy::modulo_one)]
                    {
                        branch = (start + i) % BRANCHES;
                    }
                    match branch {
                        #[allow(unreachable_code)]
                        0 => {
                            let mask = 1 << branch;
                            if disabled & mask == mask {
                                continue;
                            }
                            let (fut, ..) = &mut *futures;
                            let mut fut = unsafe { Pin::new_unchecked(fut) };
                            let out = match Future::poll(fut, cx) {
                                Ready(out) => out,
                                Pending => {
                                    is_pending = true;
                                    continue;
                                }
                            };
                            disabled |= mask;
                            #[allow(unused_variables)]
                            #[allow(unused_mut)]
                            match &out {
                                val => {}
                                _ => continue,
                            }
                            return Ready(__tokio_select_util::Out::_0(out));
                        }
                        #[allow(unreachable_code)]
                        1 => {
                            let mask = 1 << branch;
                            if disabled & mask == mask {
                                continue;
                            }
                            let (_, fut, ..) = &mut *futures;
                            let mut fut = unsafe { Pin::new_unchecked(fut) };
                            let out = match Future::poll(fut, cx) {
                                Ready(out) => out,
                                Pending => {
                                    is_pending = true;
                                    continue;
                                }
                            };
                            disabled |= mask;
                            #[allow(unused_variables)]
                            #[allow(unused_mut)]
                            match &out {
                                val => {}
                                _ => continue,
                            }
                            return Ready(__tokio_select_util::Out::_1(out));
                        }
                        _ => {
                            unreachable!("reaching this means there probably is an off by one bug");
                        }
                    }
                }
                if is_pending {
                    Pending
                } else {
                    Ready(__tokio_select_util::Out::Disabled)
                }
            })
            .await
        };
        match output {
            __tokio_select_util::Out::_0(val) => {
                {
                    println!("rx1 completed first with {0:?}\n", val)
                };
            }
            __tokio_select_util::Out::_1(val) => {
                {
                    println!("rx2 completed first with {0:?}\n", val)
                };
            }
            __tokio_select_util::Out::Disabled => {
                panic!("all branches are disabled and there is no else branch")
            }
            _ => {
                unreachable!("failed to match bind");
            }
        }
    }
}

// cd 到 lib.rs目录下
// 执行 cargo expand --lib
// 得到 expand过后的代码
//
/// mod _08_select {
///     use tokio::sync::oneshot;
///     async fn select_demo() {
///         let (tx1, rx1) = oneshot::channel();
///         let (tx2, rx2) = oneshot::channel();
///         tokio::spawn(async {
///             let _ = tx1.send("one");
///         });
///         tokio::spawn(async {
///             let _ = tx2.send("two");
///         });
///         {
///             #[doc(hidden)]
///             mod __tokio_select_util {
///                 pub(super) enum Out<_0, _1> {
///                     _0(_0),
///                     _1(_1),
///                     Disabled,
///                 }
///                 pub(super) type Mask = u8;
///             }
///             use ::tokio::macros::support::Future;
///             use ::tokio::macros::support::Pin;
///             use ::tokio::macros::support::Poll::{Ready, Pending};
///             const BRANCHES: u32 = 2;
///             let mut disabled: __tokio_select_util::Mask = Default::default();
///             if !true {
///                 let mask: __tokio_select_util::Mask = 1 << 0;
///                 disabled |= mask;
///             }
///             if !true {
///                 let mask: __tokio_select_util::Mask = 1 << 1;
///                 disabled |= mask;
///             }
///             let mut output = {
///                 let futures_init = (rx1, rx2);
///                 let mut futures = (
///                     ::tokio::macros::support::IntoFuture::into_future(futures_init.0),
///                     ::tokio::macros::support::IntoFuture::into_future(futures_init.1),
///                 );
///                 let mut futures = &mut futures;
///                 ::tokio::macros::support::poll_fn(|cx| {
///                     match ::tokio::macros::support::poll_budget_available(cx) {
///                         ::core::task::Poll::Ready(t) => t,
///                         ::core::task::Poll::Pending => {
///                             return ::core::task::Poll::Pending;
///                         }
///                     };
///                     let mut is_pending = false;
///                     let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
///                     for i in 0..BRANCHES {
///                         let branch;
///                         #[allow(clippy::modulo_one)]
///                         {
///                             branch = (start + i) % BRANCHES;
///                         }
///                         match branch {
///                             #[allow(unreachable_code)]
///                             0 => {
///                                 let mask = 1 << branch;
///                                 if disabled & mask == mask {
///                                     continue;
///                                 }
///                                 let (fut, ..) = &mut *futures;
///                                 let mut fut = unsafe { Pin::new_unchecked(fut) };
///                                 let out = match Future::poll(fut, cx) {
///                                     Ready(out) => out,
///                                     Pending => {
///                                         is_pending = true;
///                                         continue;
///                                     }
///                                 };
///                                 disabled |= mask;
///                                 #[allow(unused_variables)] #[allow(unused_mut)]
///                                 match &out {
///                                     val => {}
///                                     _ => continue,
///                                 }
///                                 return Ready(__tokio_select_util::Out::_0(out));
///                             }
///                             #[allow(unreachable_code)]
///                             1 => {
///                                 let mask = 1 << branch;
///                                 if disabled & mask == mask {
///                                     continue;
///                                 }
///                                 let (_, fut, ..) = &mut *futures;
///                                 let mut fut = unsafe { Pin::new_unchecked(fut) };
///                                 let out = match Future::poll(fut, cx) {
///                                     Ready(out) => out,
///                                     Pending => {
///                                         is_pending = true;
///                                         continue;
///                                     }
///                                 };
///                                 disabled |= mask;
///                                 #[allow(unused_variables)] #[allow(unused_mut)]
///                                 match &out {
///                                     val => {}
///                                     _ => continue,
///                                 }
///                                 return Ready(__tokio_select_util::Out::_1(out));
///                             }
///                             _ => {
///                                 ::core::panicking::panic_fmt(
///                                     format_args!(
///                                         "internal error: entered unreachable code: {0}",
///                                         format_args!(
///                                             "reaching this means there probably is an off by one bug",
///                                         ),
///                                     ),
///                                 );
///                             }
///                         }
///                     }
///                     if is_pending {
///                         Pending
///                     } else {
///                         Ready(__tokio_select_util::Out::Disabled)
///                     }
///                 })
///                     .await
///             };
///             match output {
///                 __tokio_select_util::Out::_0(val) => {
///                     {
///                         ::std::io::_print(
///                             format_args!("rx1 completed first with {0:?}\n", val),
///                         );
///                     };
///                 }
///                 __tokio_select_util::Out::_1(val) => {
///                     {
///                         ::std::io::_print(
///                             format_args!("rx2 completed first with {0:?}\n", val),
///                         );
///                     };
///                 }
///                 __tokio_select_util::Out::Disabled => {
///                     ::core::panicking::panic_fmt(
///                         format_args!(
///                             "all branches are disabled and there is no else branch",
///                         ),
///                     );
///                 }
///                 _ => {
///                     ::core::panicking::panic_fmt(
///                         format_args!(
///                             "internal error: entered unreachable code: {0}",
///                             format_args!("failed to match bind"),
///                         ),
///                     );
///                 }
///             }
///         }
///     }
/// }

async fn some_operation() -> String {
    // Compute value here
    "done".to_string()
}

#[tokio::test]
async fn cancellation_by_drop() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        // Select on the operation and the oneshot's
        // `closed()` notification.
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                // `some_operation()` is canceled, the
                // task completes and `tx1` is dropped.
                // println!("rx1 received closed notify and dropped");
            }
        }
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(());
        }

        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(());
        }

        Poll::Pending
    }
}

#[tokio::test]
async fn select_simple_impl() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    // use tx1 and tx2

    MySelect { rx1, rx2 }.await;
}
// <pattern> = <async expression> => <handler>,
// 复杂的select 匹配对象

#[tokio::test]
async fn select_complicated_1() {
    let (tx, rx) = oneshot::channel();

    // Spawn a task that sends a message over the oneshot
    tokio::spawn(async move {
        tx.send("done").unwrap();
    });

    tokio::select! {
        socket = TcpStream::connect("localhost:3465") => {
            println!("Socket connected {:?}", socket);
        }
        msg = rx => {
            println!("received message first {:?}", msg);
        }
    }
}

#[tokio::test]
async fn select_complicated_2() -> io::Result<()> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send(()).unwrap();
    });

    let mut listener = TcpListener::bind("localhost:3465").await?;

    tokio::select! {
        _ = async {
            loop {
                let (socket, _) = listener.accept().await?;
                tokio::spawn(async move { process(socket) });
            }

            // Help the rust type inferencer out
            Ok::<_, io::Error>(())
        } => {}
        _ = rx => {
            println!("terminating accept loop");
        }
    }

    Ok(())
}

async fn process(p0: TcpStream) {
    ()
}

async fn computation1() -> String {
    // .. computation
    "computation1".to_string()
}

async fn computation2() -> String {
    // .. computation
    "computation2".to_string()
}

#[tokio::test]
async fn select_return_same() {
    let out = tokio::select! {
        res1 = computation1() => res1,
        res2 = computation2() => res2,
    };

    println!("Got = {}", out);
}

// ? 在 select! 分支里是“短路”行为，直接结束整个 select!。，咩咩咩咩咩咩。
#[tokio::test]
async fn select_errors() -> io::Result<()> {
    // [setup `rx` oneshot channel]
    let (tx, rx): (Sender<()>, Receiver<()>) = oneshot::channel();
    // tokio::spawn(async move {
    //     tx.send(()).unwrap();
    // });
    let listener = TcpListener::bind("localhost:3465").await?;

    tokio::select! {
        res = async {
            loop {
                // 手动返回一个 io::Error
                // let err = Err(io::Error::new(io::ErrorKind::Other, "这是一个手动定义的错误"));
                // err?
                // 如果这里出错 这里的错误信息就会传递给 下面 handle的 res?做判断 报错返回
                let (socket, _) = listener.accept().await?;
                tokio::spawn(async move { process(socket) });
            }

            // Help the rust type inferencer out
            Ok::<_, io::Error>(())
        } => {
            res?; // 如果出错，整个 select! 返回 Err
        }
        _ = rx => {
            println!("terminating accept loop");
        }
    }

    Ok(())
}

#[tokio::test]
async fn pattern_matching() {
    let (mut tx1, mut rx1): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(128);
    let (mut tx2, mut rx2): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(128);

    tokio::spawn(async move {
        // Do something w/ `tx1` and `tx2`
    });

    tokio::select! {
        // If a channel closes, recv() returns None
        // 所以及不满足rx1.recv 也不满足rx2.recv 只能到else逻辑中了
        Some(v) = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        }
        Some(v) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }
        else => { // 此处支持默认的
            println!("Both channels closed");
        }
    }
}

// select 不要求 一个spawned async 拥有其自己的数据 能依赖外部的数据
// 能被多个不可变的引用 或者 一个可变的引用
async fn race(data: &[u8], addr1: SocketAddr, addr2: SocketAddr) -> io::Result<()> {
    tokio::select! {
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr1).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error>(())
        } => {}
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr2).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error>(())
        } => {}
        else => {}
    };

    Ok(())
}

#[tokio::test]
async fn select_out() {
    let (tx1, rx1): (Sender<String>, Receiver<String>) = oneshot::channel();
    let (tx2, rx2): (Sender<String>, Receiver<String>) = oneshot::channel();

    let mut out = String::new();

    tokio::spawn(async move {
        // Send values on `tx1` and `tx2`.
        tx1.send("tx1".to_string());
        tx2.send("tx2".to_string());
    });

    tokio::select! {
        _ = rx1 => {
            out.push_str("rx1 completed");
        }
        _ = rx2 => {
            out.push_str("rx2 completed");
        }
    }

    println!("{}", out);
}

// 虽然select 是loop实现的 但是他取哪个branch是随机的 并不是一味的前后执行
#[tokio::test]
async fn select_loop() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);

    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => { break }
        };

        println!("Got {:?}", msg);
    }

    println!("All channels have been closed.");
}

async fn action() {
    // Some asynchronous logic
}

#[tokio::test]
async fn select_resume() {
    let (mut tx, mut rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel(128);

    let operation = action();
    // tokio::pin!(operation); 的作用是
    // 将一个 Future 固定（pin）在内存中，以便后续安全地使用它，
    // 尤其是在 select! 或循环中多次 poll 时。
    // 宏会把你的 Future 包装成 Pin<&mut T>。
    // 这样你可以在 select! 或循环中反复 poll，而不会违反 Rust 的安全规则。
    // 这里 operation 是一个 Future，如果不 pin，它可能在 select! 展开后被移动，编译器会报错。

    // The thing to note is that, to .await a reference,
    // the value being referenced must be pinned or implement Unpin.
    tokio::pin!(operation);

    loop {
        tokio::select! {
            _ = &mut operation => break,
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    break;
                }
            }
        }
    }
}

async fn action1(input: Option<i32>) -> Option<String> {
    // If the input is `None`, return `None`.
    // This could also be written as `let i = input?;`
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    // async logic here
    Some(i.to_string())
}
// Modifying a branch
#[tokio::test]
async fn modify_branch() {
    let (mut tx, mut rx) = mpsc::channel(128);

    let mut done = false;
    let operation = action1(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;
                println!("enter branch1");
                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            }
            Some(v) = rx.recv() => {
                println!("enter branch2 {}",v);
                if v % 2 == 0 {
                    println!("enter branch2 even");
                    // `.set` is a method on `Pin`.
                    operation.set(action1(Some(v)));
                    done = false;
                }
            }
        }
    }
}

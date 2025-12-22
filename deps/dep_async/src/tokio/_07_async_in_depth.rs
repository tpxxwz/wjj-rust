use futures::executor::block_on;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, mpsc};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

pub struct Delay {
    pub when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            let when = self.when;

            // Spawn a timer thread.
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });
            Poll::Pending
        }
    }
}

#[tokio::test]
async fn my_future() {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "done");
}

enum MainFuture {
    // Initialized, never polled
    State0,
    // Waiting on `Delay`, i.e. the `future.await` line.
    State1(Delay),
    // The future has completed.
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        use MainFuture::*;

        loop {
            match *self {
                State0 => {
                    let when = Instant::now() + Duration::from_millis(10);
                    let future = Delay { when };
                    *self = State1(future);
                }
                State1(ref mut my_future) => match Pin::new(my_future).poll(cx) {
                    Poll::Ready(out) => {
                        assert_eq!(out, "done");
                        *self = Terminated;
                        return Poll::Ready(());
                    }
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                },
                Terminated => {
                    panic!("future polled after completion")
                }
            }
        }
    }
}

use futures::task;
use futures::task::ArcWake;
use tokio::sync::Notify;

// 用VecDeque Task 和 for循环来实现 任务做完的异步
struct MiniTokio {
    tasks: VecDeque<Task>,
}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl MiniTokio {
    fn new() -> MiniTokio {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }

    /// Spawn a future onto the mini-tokio instance.
    /// Send 的作用
    ///
    /// 保证这个 Future 可以在线程之间安全移动。
    /// 如果运行时（比如 Tokio）是多线程的，任务可能会被调度到其他线程执行，必须满足 Send。
    /// 例如，如果 Future 内部包含 Rc（不是线程安全），就不能是 Send。
    ///
    /// 'static 的作用
    ///
    /// 保证这个 Future 不包含任何非 'static 的引用，即它的生命周期是整个程序运行期间。
    /// 为什么需要这个？
    ///
    /// 当你 spawn 一个任务时，这个任务可能在当前作用域结束后仍然运行。
    /// 如果 Future 持有一个短生命周期的引用（比如借用局部变量），一旦作用域结束，引用就悬空，导致未定义行为。
    ///
    /// 'static 确保 Future 完全拥有它的数据（比如通过 Arc、Box），不会依赖外部作用域。
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    fn run(&mut self) {
        /// Waker 是什么？
        ///
        /// Waker 是 Rust 异步模型中的核心，用来通知执行器“这个 Future 可以继续执行了，请再次调用 poll”。
        /// 每次 poll 都会传入一个 Context，里面包含 Waker。
        /// 当 Future 需要挂起时，它会保存 Waker，并在条件满足时调用 wake()。

        /// task::noop_waker() 是来自 futures crate 的一个工具函数，
        /// 用来创建一个 “空操作” Waker，也就是一个不会真正唤醒任务的 Waker。
        ///
        let waker = task::noop_waker();

        /// Context::from_waker(&waker) 的作用是 根据一个 Waker 创建一个 Context 对象，用于传递给 poll 方法。
        /// ✅ 为什么需要 Context？
        ///
        /// Rust 的异步模型要求每次调用 poll 时传入一个 &mut Context。
        /// Context 里面包含：
        ///
        /// Waker：用于唤醒任务，让执行器再次调用 poll。
        ///
        ///
        /// 当 Future 返回 Poll::Pending 时，它可以通过 cx.waker() 获取 Waker，并在条件满足时调用 wake()。
        let mut cx = Context::from_waker(&waker);

        /// 不断空跑 for 循环 知道完成 来实现的异步
        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}
#[test]
fn mini_tokio_test() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };

        let out = future.await;
        assert_eq!(out, "done");
    });

    mini_tokio.run();
}

struct Delay1 {
    when: Instant,
    // This is Some when we have spawned a thread, and None otherwise.
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay1 {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // Check the current instant. If the duration has elapsed, then
        // this future has completed so we return `Poll::Ready`.
        if Instant::now() >= self.when {
            return Poll::Ready(());
        }

        // The duration has not elapsed. If this is the first time the future
        // is called, spawn the timer thread. If the timer thread is already
        // running, ensure the stored `Waker` matches the current task's waker.
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();

            // Check if the stored waker matches the current task's waker.
            // This is necessary as the `Delay` future instance may move to
            // a different task between calls to `poll`. If this happens, the
            // waker contained by the given `Context` will differ and we
            // must update our stored waker to reflect this change.
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());

            // This is the first time `poll` is called, spawn the timer thread.
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                // The duration has elapsed. Notify the caller by invoking
                // the waker.
                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }

        // By now, the waker is stored and the timer thread is started.
        // The duration has not elapsed (recall that we checked for this
        // first thing), ergo the future has not completed so we must
        // return `Poll::Pending`.
        //
        // The `Future` trait contract requires that when `Pending` is
        // returned, the future ensures that the given waker is signalled
        // once the future should be polled again. In our case, by
        // returning `Pending` here, we are promising that we will
        // invoke the given waker included in the `Context` argument
        // once the requested duration has elapsed. We ensure this by
        // spawning the timer thread above.
        //
        // If we forget to invoke the waker, the task will hang
        // indefinitely.
        Poll::Pending
    }
}

struct MiniTokio1 {
    scheduled: mpsc::Receiver<Arc<Task1>>,
    sender: mpsc::Sender<Arc<Task1>>,
}

struct TaskFuture {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    poll: Poll<()>,
}

// 这就是一个 迷你执行器的核心机制：
//
// Task1 封装了一个 Future。
// 当 Future Pending 时，注册 waker。
// 当条件满足时，waker 调用 wake_by_ref() → schedule() → 执行器再次 poll()。
//
//
// 这样就实现了 事件驱动的异步调度，而不是忙轮询。
struct Task1 {
    // The `Mutex` is to make `Task` implement `Sync`. Only
    // one thread accesses `task_future` at any given time.
    // The `Mutex` is not required for correctness. Real Tokio
    // does not use a mutex here, but real Tokio has
    // more lines of code than can fit in a single tutorial
    // page.

    // Sync 是 Rust 标准库里的一个 标记 trait，它的作用是：
    // 如果一个类型是 Sync，那么它可以安全地在多个线程中通过共享引用（&T）同时访问。
    // Sync 保证 共享引用跨线程安全。
    // Send 保证 所有权跨线程安全。
    // 两者结合，Rust 实现了零成本的并发安全
    task_future: Mutex<TaskFuture>,
    executor: mpsc::Sender<Arc<Task1>>,
}
impl Task1 {
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone());
    }

    fn poll(self: Arc<Self>) {
        // Create a waker from the `Task` instance. This
        // uses the `ArcWake` impl from above.
        // 首先 Task1::spawn future的时候第一次把task1推到了MiniTokio1里的channel里面
        // 然后recv 一路调用poll
        // 第一次调用到最终future 也就是Delay的Future 但此时时间太快还没来得及做完
        // 开了个线程异步倒计时唤醒 Context里的 waker.wake() 此时返回了Poll::Pending 还是塞进了TaskFuture的poll字段
        // 异步倒计时唤醒 Context里的 waker.wake()后
        // 由于实现了ArcWake的wake_by_ref方法 Task1 arc_self.schedule() 将Task1再次发送到MiniTokio1里的channel里面
        // MiniTokio1里的Receiver接收来自 channel里面的Task1 再次执行 Task1的poll方法
        // 此时也new了一个新的Context 本身Task1再次被塞进这个新的Context 调用task_future的poll方法
        // 发现self.poll.is_pending()还是pending状态的
        // if self.poll.is_pending() {
        //     self.poll = self.future.as_mut().poll(cx);
        // }
        // 再次执行真正的future 也就是Delay delay里面返回ready前的就绪的逻辑都执行了后
        // 这是Delay返回Poll::Ready 并赋值到了TaskFuture的poll字段
        // 到此 这一整个流程结束
        // 要保证线程倒计时重新唤醒后 Poll::Ready 前面的逻辑都能执行完
        // TaskFuture的poll 字段只有两次未Ready的情况下 让其调用到了最终的future 的poll方法
        // TaskFuture的poll在为Poll::Ready的时候 其实是没做判断了 用不到了
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        // No other thread ever tries to lock the task_future
        let mut task_future = self.task_future.try_lock().unwrap();

        // Poll the inner future
        task_future.poll(&mut cx);
    }

    // Spawns a new task with the given future.
    //
    // Initializes a new Task harness containing the given future and pushes it
    // onto `sender`. The receiver half of the channel will get the task and
    // execute it.
    fn spawn<F>(future: F, sender: &mpsc::Sender<Arc<Task1>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task1 {
            task_future: Mutex::new(TaskFuture::new(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}

impl ArcWake for Task1 {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

impl TaskFuture {
    fn new(future: impl Future<Output = ()> + Send + 'static) -> TaskFuture {
        TaskFuture {
            future: Box::pin(future),
            poll: Poll::Pending,
        }
    }

    fn poll(&mut self, cx: &mut Context<'_>) {
        // Spurious wake-ups are allowed, even after a future has
        // returned `Ready`. However, polling a future which has
        // already returned `Ready` is *not* allowed. For this
        // reason we need to check that the future is still pending
        // before we call it. Failure to do so can lead to a panic.
        if self.poll.is_pending() {
            self.poll = self.future.as_mut().poll(cx);
        }
    }
}

impl MiniTokio1 {
    /// Initialize a new mini-tokio instance.
    fn new() -> MiniTokio1 {
        let (sender, scheduled) = mpsc::channel();

        MiniTokio1 { scheduled, sender }
    }

    /// Spawn a future onto the mini-tokio instance.
    ///
    /// The given future is wrapped with the `Task` harness and pushed into the
    /// `scheduled` queue. The future will be executed when `run` is called.
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task1::spawn(future, &self.sender);
    }

    fn run(&self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

// 直接运行测试会不显示println!的 运行时候需要加上 --nocapture
#[test]
fn mini_tokio1_test() {
    let mut mini_tokio = MiniTokio1::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(100);
        let future = Delay1 { when, waker: None };
        let out = future.await;
        assert_eq!(out, ());
    });
    mini_tokio.run();
}

async fn delay(dur: Duration) -> &'static str {
    let when = Instant::now() + dur;
    // This utility provides a basic task notification mechanism.
    // It handles the details of wakers,
    // including making sure that the recorded waker matches the current task.
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();

    thread::spawn(move || {
        let now = Instant::now();

        if now < when {
            thread::sleep(when - now);
        }

        notify_clone.notify_one();
    });

    notify.notified().await;
    println!("delay done");
    "done"
}

#[test]
fn notify_test() {
    let mut mini_tokio = MiniTokio1::new();

    mini_tokio.spawn(async {
        let out = delay(Duration::from_secs(1)).await;
        assert_eq!(out, "done");
    });
    mini_tokio.run();
}

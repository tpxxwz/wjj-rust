use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread;
use std::time::{Duration, Instant};

// 使用示例

// ================== Sleep Future ==================
struct Sleep {
    when: Instant,
    waker: Arc<Mutex<Option<Waker>>>,
    started: bool,
}

impl Sleep {
    fn new(dur: Duration) -> Self {
        Sleep {
            when: Instant::now() + dur,
            waker: Arc::new(Mutex::new(None)),
            started: false,
        }
    }
}

impl Future for Sleep {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if Instant::now() >= self.when {
            return Poll::Ready(());
        }

        // 注册 waker
        *self.waker.lock().unwrap() = Some(cx.waker().clone());

        // 启动后台线程（只启动一次）
        if !self.started {
            self.started = true;
            let when = self.when;
            let waker_clone = self.waker.clone();
            thread::spawn(move || {
                let now = Instant::now();
                if when > now {
                    thread::sleep(when - now);
                }
                if let Some(w) = waker_clone.lock().unwrap().take() {
                    w.wake(); // 唤醒任务
                }
            });
        }

        Poll::Pending
    }
}

// ================== Runtime ==================
struct Runtime {
    tasks: Arc<Mutex<VecDeque<Pin<Box<dyn Future<Output = ()>>>>>>,
    condvar: Arc<Condvar>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
            condvar: Arc::new(Condvar::new()),
        }
    }

    fn spawn<F>(&mut self, fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.tasks.lock().unwrap().push_back(Box::pin(fut));
    }

    fn run(&mut self) {
        let waker = self.create_waker();
        let mut cx = Context::from_waker(&waker);

        loop {
            let mut tasks = self.tasks.lock().unwrap();
            if tasks.is_empty() {
                break; // 所有任务完成
            }

            let mut pending = VecDeque::new();
            while let Some(mut task) = tasks.pop_front() {
                if task.as_mut().poll(&mut cx) == Poll::Pending {
                    pending.push_back(task);
                }
            }

            if pending.is_empty() {
                break; // 全部完成
            }

            *tasks = pending;
            tasks = self.condvar.wait(tasks).unwrap(); // 等待唤醒
        }
    }

    fn create_waker(&self) -> Waker {
        use std::task::{RawWaker, RawWakerVTable};

        fn clone(data: *const ()) -> RawWaker {
            RawWaker::new(data, &VTABLE)
        }

        fn wake(data: *const ()) {
            // 转回 Arc
            let arc: Arc<(Mutex<VecDeque<Pin<Box<dyn Future<Output = ()>>>>>, Condvar)> =
                unsafe { Arc::from_raw(data as *const _) };

            // 通知 condvar
            arc.1.notify_one();

            // 防止 Arc 被 drop（因为 from_raw 会减少引用计数）
            std::mem::forget(arc);
        }

        fn wake_by_ref(data: *const ()) {
            wake(data);
        }
        fn drop(_: *const ()) {}

        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

        let data = Arc::into_raw(Arc::new((self.tasks.clone(), self.condvar.clone()))) as *const ();
        unsafe { Waker::from_raw(RawWaker::new(data, &VTABLE)) }
    }
}

// ================== main ==================
fn main() {
    let mut rt = Runtime::new();
    let start = Instant::now();
    println!("starting!");

    for _ in 0..1_000_000 {
        rt.spawn(async {
            Sleep::new(Duration::from_millis(10)).await;
        });
    }

    rt.run();
    println!("all done");
    let duration = start.elapsed();
    println!("elapsed: {:?}", duration);
}

// core::pin 是 Rust 标准库中的一个模块，主要用于 固定（pin）数据在内存中的位置，防止它在异步或自引用场景中被移动。
// 它是实现 Future 和异步安全的重要基础。

// 为什么需要 Pin？
//
// Rust 默认允许值在内存中移动（比如 Box<T> 可以被移动）。
// 但是某些类型（尤其是 Future 或自引用结构）在移动后会导致内部指针失效。
// Pin 的作用就是：保证一个值在内存中不会再被移动。

use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};

// PhantomPinned + Pin 组合可以让编译器禁止移动这个结构体，保证安全。
struct MyFuture {
    data: String,
    ptr: *const String,  // 自引用指针
    _pin: PhantomPinned, // 标记不可移动 PhantomPinned 是一个标记类型，用来告诉编译器“这个结构体不能被移动”。
}

impl MyFuture {
    fn new(data: String) -> Self {
        MyFuture {
            data,
            ptr: std::ptr::null(), // 在 new() 初始化的时候，把 ptr 字段设置为 空指针，即指向 null，而不是指向任何有效的内存地址。
            _pin: PhantomPinned,
        }
    }

    // 背景
    //
    // self 的类型是 Pin<&mut Self>，即一个被固定（pinned）的可变引用。
    // Pin 的作用是保证数据不会被移动，防止自引用结构出现悬空指针。
    // 但是，Pin 会限制你直接获取普通的 &mut Self，因为那样可能导致移动。
    fn init(self: Pin<&mut Self>) {
        // 它会 绕过 Pin 的保护，直接返回普通的 &mut T。
        // 为什么是 unsafe？
        //
        // 因为如果你在拿到 &mut T 后移动了这个值，就破坏了 Pin 的保证，可能导致未定义行为（UB）。
        //
        //
        // 所以，调用者必须保证：
        //
        // 不会移动这个值，只修改字段是安全的。
        let this = unsafe { self.get_unchecked_mut() };
        this.ptr = &this.data; // 在对象被 Pin 后，再把 ptr 设置为 &self.data
    }
}

impl Future for MyFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("data via ptr: {}", unsafe { &*self.ptr });
        Poll::Ready(())
    }
}

#[test]
fn use_pin() {
    let mut f = Box::pin(MyFuture::new("Hello Pin!".into()));
    f.as_mut().init(); // 初始化自引用
    futures::executor::block_on(f); // 安全运行
}

struct BadFuture {
    data: String,
    ptr: *const String, // 自引用指针
}

impl BadFuture {
    fn new(data: String) -> Self {
        BadFuture {
            data,
            ptr: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        self.ptr = &self.data; // 自引用
    }
}

impl Future for BadFuture {
    type Output = ();
    fn poll(mut self: std::pin::Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("data via ptr: {}", unsafe { &*self.ptr });
        Poll::Ready(())
    }
}
// UB 是 Undefined Behavior
#[test]
fn without_pin() {
    let mut f = BadFuture::new("Hello".into());
    f.init();

    let mut f2 = f; // 移动发生
    f2.init();

    println!("Pointer address: {:p}", f2.ptr);

    unsafe {
        println!("Pointer content: {}", *f2.ptr);
    }
}

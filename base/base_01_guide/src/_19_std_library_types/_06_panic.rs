// panic!
// panic! 宏可用于生成一个 panic 并开始展开其栈。
// 在展开过程中，运行时会通过调用该线程所有对象的析构函数来释放线程拥有的所有资源。
//
// 由于我们处理的是只有一个线程的程序，panic! 会导致程序报告 panic 消息并退出。

// 重新实现整数除法（/）
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // 除以零会触发 panic
        panic!("除以零");
    } else {
        dividend / divisor
    }
}

// `main` 任务
#[wjj_lib::gen_test]
fn main() {
    // 堆分配的整数
    let _x = Box::new(0i32);

    // 这个操作将触发任务失败
    division(3, 0);

    println!("这个点不会被执行到！");

    // `_x` 应该在此处被销毁
}



// 让我们验证 panic! 不会导致内存泄漏。
// $ rustc panic.rs && valgrind ./panic
// ==4401== Memcheck, a memory error detector
// ==4401== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
// ==4401== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
// ==4401== Command: ./panic
// ==4401==
// thread '<main>' panicked at 'division by zero', panic.rs:5
// ==4401==
// ==4401== HEAP SUMMARY:
// ==4401==     in use at exit: 0 bytes in 0 blocks
// ==4401==   total heap usage: 18 allocs, 18 frees, 1,648 bytes allocated
// ==4401==
// ==4401== All heap blocks were freed -- no leaks are possible
// ==4401==
// ==4401== For counts of detected and suppressed errors, rerun with: -v
// ==4401== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
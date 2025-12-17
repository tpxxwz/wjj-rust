// abort 和 unwind
// 上一节介绍了错误处理机制 panic。可以根据 panic 设置有条件地编译不同的代码路径。当前可用的值有 unwind 和 abort。
//
// 基于之前的柠檬水示例，我们明确使用 panic 策略来执行不同的代码行。

fn drink1(beverage: &str) {
    // 你不应该喝太多含糖饮料。
    if beverage == "柠檬水" {
        if cfg!(panic = "中止") {
            println!("这不是你的派对。快跑！！！！");
        } else {
            println!("快吐出来！！！！");
        }
    } else {
        println!("来点清爽的{}就是我现在需要的。", beverage);
    }
}

#[test]
fn main1() {
    drink1("水");
    drink1("柠檬水");
}

// 这里是另一个示例，重点是重写 drink() 函数并明确使用 unwind 关键字。

#[cfg(panic = "unwind")]
fn ah() {
    println!("快吐出来！！！！");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("这不是你的派对。快跑！！！！");
}

fn drink2(beverage: &str) {
    if beverage == "柠檬水" {
        ah();
    } else {
        println!("来点清爽的{}就是我现在需要的。", beverage);
    }
}

#[test]
fn main2() {
    drink2("水");
    drink2("柠檬水");
}

// 这里是另一个示例，重点是重写 drink() 函数并明确使用 unwind 关键字。

#[cfg(panic = "unwind")]
fn ah() {
    println!("快吐出来！！！！");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("这不是你的派对。快跑！！！！");
}

fn drink(beverage: &str) {
    if beverage == "柠檬水" {
        ah();
    } else {
        println!("来点清爽的{}就是我现在需要的。", beverage);
    }
}

fn main() {
    drink("水");
    drink("柠檬水");
}

crate::gen_test!(main);
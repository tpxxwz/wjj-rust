// 04_ownership.rs
// 所有权

fn main() {
    // 变量作用域
    {
        let s = "hello"; // s在这里开始有效
        println!("{}", s);
    } // s在这里离开作用域，不再有效

    // 移动语义
    let s1 = String::from("hello");
    let s1 = "hello";
    let s2 = s1; // s1被移动到s2，s1不再有效
    println!("{}", s2);

    // 克隆
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1被克隆到s2，s1仍然有效
    println!("s1 = {}, s2 = {}", s1, s2);

    // 函数与所有权
    let s = String::from("hello");
    takes_ownership(s); // s的所有权被移动到函数中
    // println!("{}", s); // 这里会报错，因为s已经无效

    let x = 5;
    makes_copy(x); // x是Copy类型，不会被移动
    println!("{}", x); // x仍然有效

    // 返回值与所有权
    let s1 = gives_ownership(); // 函数返回值所有权转移给s1
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2所有权转移，返回值所有权转移给s3
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string离开作用域，被自动释放

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer离开作用域，没有特殊操作

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
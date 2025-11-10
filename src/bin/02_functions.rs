// 02_functions.rs
// 函数

fn main() {
    // 简单函数
    print_hello();

    // 带参数的函数
    print_sum(5, 10);

    // 带返回值的函数
    let result = add(3, 4);
    println!("3 + 4 = {}", result);

    // 函数表达式
    let square = |x: i32| -> i32 { x * x };
    println!("5的平方是: {}", square(5));
}

// 无参数无返回值的函数
fn print_hello() {
    println!("Hello!");
}

// 带参数无返回值的函数
fn print_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

// 带参数和返回值的函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn add_ownership_transfer(a: i32) {} // 移交所有权

// 不可变引用和可变引用 都算作是借用 获取所有权 算移交所有权

// 不可变引用
// fn add_immutable_ref(a: &i32) ->&String{
//     let s = String::from("hello");
//     &s                               // 不能只返回指针 指针对应的堆的所有权属于当前方法 用完了就销毁了 饭回指针不行
// }

// 可变引用
fn add_mutable_ref(a: &mut i32) {}

// 获取所有权
fn add_taking_ownership() -> String {
    let s = String::from("hello");
    s
}

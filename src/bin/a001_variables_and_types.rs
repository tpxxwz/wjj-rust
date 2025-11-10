// 01_variables_and_types.rs
// 变量与数据类型

fn main() {
    // 变量声明
    let x = 5; // 使用let声明变量，类型自动推断为i32
    println!("x = {}", x);

    // 可变变量
    let mut y = 10; // 使用mut关键字声明可变变量
    y = 20;
    println!("y = {}", y);

    // 常量
    const MAX_POINTS: u32 = 100_000; // 使用const声明常量，必须显式指定类型
    println!("MAX_POINTS = {}", MAX_POINTS);

    // 标量类型
    // 整数类型
    let a: i32 = 42;
    let b: u64 = 1_000_000;
    // 浮点类型
    let c: f64 = 3.14;
    // 布尔类型
    let d: bool = true;
    // 字符类型
    let e: char = 'R';

    // 复合类型
    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 解构元组
    println!("x = {}, y = {}, z = {}", x, y, z);

    // 数组
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    println!("first element = {}", first);

    // 类型转换
    let int_val = 5;
    let float_val = int_val as f64;
    println!("float_val = {}", float_val);
}
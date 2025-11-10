fn main() {
    println!("=== 引用 (&T 和 &mut T) 学习示例 ===\n");

    // 不可变引用 (&T)
    immutable_references();
    
    // 可变引用 (&mut T)
    mutable_references();
    
    // 引用作为函数参数
    references_as_function_parameters();
    
    // 悬垂引用 (Dangling References) - 编译时错误
    // dangling_reference(); // 这将导致编译错误
    
    // 引用的规则
    reference_rules();
}

fn immutable_references() {
    println!("=== 不可变引用 (&T) ===");
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("字符串 '{}' 的长度是 {}.", s1, len);
    
    let x = 5;
    let y = &x;
    println!("x = {}, y = {}", x, *y);
    
    println!();
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 离开作用域，但因为它不拥有所有权，所以什么也不会发生

fn mutable_references() {
    println!("=== 可变引用 (&mut T) ===");
    
    let mut s = String::from("hello");
    println!("修改前: {}", s);
    
    change(&mut s);
    println!("修改后: {}", s);
    
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    println!("x = {}", x);
    
    println!();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn references_as_function_parameters() {
    println!("=== 引用作为函数参数 ===");
    
    let mut s = String::from("Rust");
    
    // 传递不可变引用
    print_string(&s);
    
    // 传递可变引用
    add_suffix(&mut s);
    print_string(&s);
    
    println!();
}

fn print_string(s: &String) {
    println!("打印字符串: {}", s);
}

fn add_suffix(s: &mut String) {
    s.push_str(" is awesome!");
}

/*
// 悬垂引用 (Dangling References) - 这段代码无法编译
fn dangling_reference() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串
    &s // 我们返回 s 的引用
} // 这里 s 离开作用域并被丢弃。它的内存被释放。
  // 危险！
*/

fn reference_rules() {
    println!("=== 引用的规则 ===");
    
    let mut s = String::from("hello");

    // 规则1: 在同一作用域内，可以有多个不可变引用
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {}, r2 = {}", r1, r2);

    // 规则2: 在同一作用域内，只能有一个可变引用
    let r3 = &mut s;
    println!("r3 = {}", r3);

    // 规则3: 不能同时拥有可变引用和不可变引用
    // let r4 = &s; // 错误: 当存在可变引用 r3 时，不能创建不可变引用
    // println!("r3 = {}, r4 = {}", r3, r4);
    
    // 引用的作用域从创建开始，到最后一次使用结束
    let mut s2 = String::from("world");
    let r4 = &s2;
    let r5 = &s2;
    println!("r4 = {}, r5 = {}", r4, r5);
    // r4 和 r5 的作用域在这里结束
    
    let r6 = &mut s2; // 现在可以创建可变引用
    println!("r6 = {}", r6);
    
    println!();
}
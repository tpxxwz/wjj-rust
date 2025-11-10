// 13_option.rs

fn main() {
    println!("=== Option<T> 学习示例 ===\n");
    
    // 1. 创建Option的几种方式
    basic_creation();
    
    // 2. 访问Option中的值
    accessing_values();
    
    // 3. 转换和组合
    transforming_options();
    
    // 4. 实用示例
    practical_examples();
}

// 1. 创建Option的几种方式
fn basic_creation() {
    println!("1. 创建Option的几种方式:");
    
    // Some - 包含值
    let some_number = Some(7);
    let some_string = Some("hello");
    
    // None - 没有值
    let absent_number: Option<i32> = None;
    
    println!("   Some(7): {:?}", some_number);
    println!("   Some(\"hello\"): {:?}", some_string);
    println!("   None: {:?}", absent_number);
    
    // 从值创建
    let x = 42;
    let option_x = Some(x);
    println!("   从值42创建: {:?}", option_x);
    
    println!();
}

// 2. 访问Option中的值
fn accessing_values() {
    println!("2. 访问Option中的值:");
    
    let some_value = Some(10);
    let no_value: Option<i32> = None;
    
    // match模式匹配
    match some_value {
        Some(value) => println!("   match - 值是: {}", value),
        None => println!("   match - 没有值"),
    }
    
    // if let语法
    if let Some(value) = some_value {
        println!("   if let - 值是: {}", value);
    }
    
    // unwrap() - 不安全，可能panic
    let unwrapped = some_value.unwrap();
    println!("   unwrap() - 值是: {}", unwrapped);
    
    // unwrap_or() - 提供默认值
    let with_default = no_value.unwrap_or(100);
    println!("   unwrap_or(100) - 值是: {}", with_default);
    
    // unwrap_or_else() - 使用闭包提供默认值
    let with_closure = no_value.unwrap_or_else(|| 200);
    println!("   unwrap_or_else(|| 200) - 值是: {}", with_closure);
    
    // expect() - 带错误信息的unwrap
    let expected = some_value.expect("应该有值");
    println!("   expect() - 值是: {}", expected);
    
    println!();
}

// 3. 转换和组合
fn transforming_options() {
    println!("3. 转换和组合Option:");
    
    let number = Some(5);
    let no_number: Option<i32> = None;
    
    // map() - 转换值
    let mapped = number.map(|x| x * 2);
    println!("   map(|x| x * 2): {:?}", mapped);
    
    // and_then() - 链式操作
    let chained = number
        .and_then(|x| Some(x + 1))
        .and_then(|x| if x > 5 { Some(x * 2) } else { None });
    println!("   and_then链式操作: {:?}", chained);
    
    // filter() - 条件过滤
    let filtered = number.filter(|&x| x > 3);
    println!("   filter(|&x| x > 3): {:?}", filtered);
    
    // or() - 提供备选
    let alternative = no_number.or(Some(999));
    println!("   or(Some(999)): {:?}", alternative);
    
    // get_or_insert() - 如果为None则插入值
    let mut mutable_option = None;
    let value = mutable_option.get_or_insert(42);
    println!("   get_or_insert(42): {:?}", mutable_option);
    
    println!();
}

// 4. 实用示例
fn practical_examples() {
    println!("4. 实用示例:");
    
    // 示例1: 安全的除法
    fn safe_divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None
        } else {
            Some(a / b)
        }
    }
    
    let result1 = safe_divide(10.0, 2.0);
    let result2 = safe_divide(10.0, 0.0);
    
    println!("   安全除法 10/2: {:?}", result1);
    println!("   安全除法 10/0: {:?}", result2);
    
    // 示例2: 查找元素
    fn find_first_even(numbers: &[i32]) -> Option<i32> {
        numbers.iter().find(|&&x| x % 2 == 0).copied()
    }
    
    let numbers = vec![1, 3, 5, 6, 8, 9];
    let even = find_first_even(&numbers);
    println!("   查找第一个偶数: {:?}", even);
    
    // 示例3: 配置读取
    #[derive(Debug)]
    struct Config {
        port: Option<u16>,
        host: Option<String>,
    }
    
    let config = Config {
        port: Some(8080),
        host: None,
    };
    
    let port = config.port.unwrap_or(3000);
    let host = config.host.as_ref().map(|h| h.as_str()).unwrap_or("localhost");
    
    println!("   配置端口: {}", port);
    println!("   配置主机: {}", host);
    
    // 示例4: 组合多个Option
    fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            (Some(x), Some(y)) => Some(x + y),
            _ => None,
        }
    }
    
    let sum = add_options(Some(5), Some(3));
    println!("   Option相加: {:?}", sum);
    
    println!();
}
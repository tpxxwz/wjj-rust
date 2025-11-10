// 14_result.rs

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() {
    println!("=== Result<T, E> 学习示例 ===\n");
    
    // 1. 创建Result的几种方式
    basic_creation();
    
    // 2. 处理Result中的值
    handling_results();
    
    // 3. 转换和组合
    transforming_results();
    
    // 4. 错误处理模式
    error_handling_patterns();
    
    // 5. 实用示例
    practical_examples();
}

// 1. 创建Result的几种方式
fn basic_creation() {
    println!("1. 创建Result的几种方式:");
    
    // Ok - 成功结果
    let ok_result: Result<i32, &str> = Ok(42);
    let ok_string: Result<String, &str> = Ok("Hello".to_string());
    
    // Err - 错误结果
    let err_result: Result<i32, &str> = Err("发生错误");
    
    println!("   Ok(42): {:?}", ok_result);
    println!("   Ok(\"Hello\".to_string()): {:?}", ok_string);
    println!("   Err(\"发生错误\"): {:?}", err_result);
    
    // 从操作创建
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    let division1 = divide(10, 2);
    let division2 = divide(10, 0);
    
    println!("   除法 10/2: {:?}", division1);
    println!("   除法 10/0: {:?}", division2);
    
    println!();
}

// 2. 处理Result中的值
fn handling_results() {
    println!("2. 处理Result中的值:");
    
    let success: Result<i32, &str> = Ok(100);
    let failure: Result<i32, &str> = Err("错误信息");
    
    // match模式匹配
    match success {
        Ok(value) => println!("   match - 成功值: {}", value),
        Err(error) => println!("   match - 错误: {}", error),
    }
    
    // if let语法
    if let Ok(value) = success {
        println!("   if let - 成功值: {}", value);
    }
    
    // unwrap() - 不安全，可能panic
    let unwrapped = success.unwrap();
    println!("   unwrap() - 成功值: {}", unwrapped);
    
    // unwrap_or() - 提供默认值
    let with_default = failure.unwrap_or(999);
    println!("   unwrap_or(999) - 值: {}", with_default);
    
    // unwrap_or_else() - 使用闭包处理错误
    let with_closure = failure.unwrap_or_else(|err| {
        println!("   处理错误: {}", err);
        0
    });
    println!("   unwrap_or_else处理结果: {}", with_closure);
    
    // expect() - 带错误信息的unwrap
    let expected = success.expect("操作应该成功");
    println!("   expect() - 成功值: {}", expected);
    
    println!();
}

// 3. 转换和组合
fn transforming_results() {
    println!("3. 转换和组合Result:");
    
    let success: Result<i32, &str> = Ok(5);
    let failure: Result<i32, &str> = Err("错误");
    
    // map() - 转换成功值
    let mapped = success.map(|x| x * 2);
    println!("   map(|x| x * 2): {:?}", mapped);
    
    // map_err() - 转换错误值
    let mapped_err = failure.map_err(|err| format!("严重错误: {}", err));
    println!("   map_err(): {:?}", mapped_err);
    
    // and_then() - 链式操作
    let chained = success
        .and_then(|x| Ok(x + 1))
        .and_then(|x| if x > 5 { Ok(x * 2) } else { Err("值太小") });
    println!("   and_then链式操作: {:?}", chained);
    
    // or() - 提供备选
    let alternative = failure.or(Ok(999));
    println!("   or(Ok(999)): {:?}", alternative);
    
    // or_else() - 使用闭包提供备选
    let or_else_result = failure.or_else(|_| Ok(777));
    println!("   or_else(): {:?}", or_else_result);
    
    println!();
}

// 4. 错误处理模式
fn error_handling_patterns() {
    println!("4. 错误处理模式:");
    
    // 自定义错误类型
    #[derive(Debug)]
    enum MyError {
        IoError(String),
        ParseError(String),
        ValidationError(String),
    }
    
    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                MyError::IoError(msg) => write!(f, "IO错误: {}", msg),
                MyError::ParseError(msg) => write!(f, "解析错误: {}", msg),
                MyError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
            }
        }
    }
    
    impl std::error::Error for MyError {}
    
    // 错误传播 - ?运算符
    fn read_file_contents(path: &str) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
    
    // 错误转换
    fn parse_number(s: &str) -> Result<i32, ParseIntError> {
        s.parse::<i32>()
    }
    
    // 组合多个可能失败的操作
    fn process_data(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let number = input.parse::<i32>()?;
        if number < 0 {
            return Err("数字不能为负".into());
        }
        Ok(number * 2)
    }
    
    // 测试错误传播
    match process_data("42") {
        Ok(result) => println!("   处理成功: {}", result),
        Err(e) => println!("   处理失败: {}", e),
    }
    
    match process_data("-1") {
        Ok(result) => println!("   处理成功: {}", result),
        Err(e) => println!("   处理失败: {}", e),
    }
    
    println!();
}

// 5. 实用示例
fn practical_examples() {
    println!("5. 实用示例:");
    
    // 示例1: 用户输入验证
    fn validate_email(email: &str) -> Result<String, String> {
        if email.contains('@') && email.contains('.') {
            Ok(email.to_string())
        } else {
            Err("无效的邮箱地址".to_string())
        }
    }
    
    let valid_email = validate_email("user@example.com");
    let invalid_email = validate_email("invalid-email");
    
    println!("   有效邮箱: {:?}", valid_email);
    println!("   无效邮箱: {:?}", invalid_email);
    
    // 示例2: 文件操作
    fn read_config() -> Result<String, io::Error> {
        // 这里使用一个可能存在的文件路径
        let mut file = File::open("/etc/hosts")?; // 通常这个文件存在
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
    
    match read_config() {
        Ok(contents) => println!("   读取配置文件成功，长度: {}字符", contents.len()),
        Err(e) => println!("   读取配置文件失败: {}", e),
    }
    
    // 示例3: 链式错误处理
    fn calculate_discount(price_str: &str, discount_str: &str) -> Result<f64, String> {
        let price = price_str.parse::<f64>()
            .map_err(|_| "价格格式错误")?;
        let discount = discount_str.parse::<f64>()
            .map_err(|_| "折扣格式错误")?;
        
        if price < 0.0 {
            return Err("价格不能为负".to_string());
        }
        if discount < 0.0 || discount > 100.0 {
            return Err("折扣必须在0-100之间".to_string());
        }
        
        Ok(price * (1.0 - discount / 100.0))
    }
    
    let discount1 = calculate_discount("100.0", "20.0");
    let discount2 = calculate_discount("abc", "20.0");
    
    println!("   计算折扣1: {:?}", discount1);
    println!("   计算折扣2: {:?}", discount2);
    
    // 示例4: 收集多个结果
    fn parse_numbers(strings: &[&str]) -> Result<Vec<i32>, ParseIntError> {
        strings.iter()
            .map(|s| s.parse::<i32>())
            .collect()
    }
    
    let numbers = vec!["1", "2", "3", "4"];
    let parsed = parse_numbers(&numbers);
    println!("   解析数字数组: {:?}", parsed);
    
    println!();
}
fn main() {
    println!("=== String 学习示例 ===\n");

    // 基础 String 使用
    basic_string();
    
    // String 的创建和初始化
    string_creation();
    
    // String 的修改操作
    string_modification();
    
    // String 的查找和替换
    string_search_and_replace();
    
    // String 的分割和连接
    string_split_and_join();
    
    // String 的转换
    string_conversions();
    
    // String 的高级用法
    advanced_string_usage();
}

fn basic_string() {
    println!("=== 基础 String 使用 ===");
    
    // 创建空 String
    let mut s1 = String::new();
    println!("空 String: '{}'，长度: {}", s1, s1.len());
    
    // 使用 to_string() 创建
    let s2 = "Hello".to_string();
    println!("使用 to_string(): '{}'", s2);
    
    // 使用 String::from() 创建
    let s3 = String::from("World");
    println!("使用 String::from(): '{}'", s3);
    
    // 使用 String::with_capacity() 创建
    let mut s4 = String::with_capacity(10);
    s4.push_str("Rust");
    println!("预分配容量的 String: '{}'，容量: {}", s4, s4.capacity());
    
    // 检查是否为空
    println!("s1 是否为空: {}", s1.is_empty());
    println!("s2 是否为空: {}", s2.is_empty());
    
    // 获取长度（字节长度）
    let unicode_str = "你好，世界";
    let s5 = unicode_str.to_string();
    println!("中文字符串: '{}'，字节长度: {}", s5, s5.len());
    println!("中文字符串字符数: {}", s5.chars().count());
    
    println!();
}

fn string_creation() {
    println!("=== String 的创建和初始化 ===");
    
    // 从字符串字面量创建
    let s1 = "Hello, Rust!".to_string();
    println!("从字符串字面量: '{}'", s1);
    
    // 从字符数组创建
    let chars = ['R', 'u', 's', 't'];
    let s2: String = chars.iter().collect();
    println!("从字符数组: '{}'", s2);
    
    // 使用 format! 宏创建
    let name = "Alice";
    let age = 30;
    let s3 = format!("My name is {} and I am {} years old.", name, age);
    println!("使用 format! 宏: '{}'", s3);
    
    // 使用 repeat 创建重复字符串
    let stars = "*".repeat(10);
    println!("重复字符串: '{}'", stars);
    
    // 从字节数组创建（需要有效的 UTF-8）
    let bytes = vec![72, 101, 108, 108, 111]; // "Hello" 的字节
    match String::from_utf8(bytes) {
        Ok(s) => println!("从字节数组: '{}'", s),
        Err(e) => println!("转换失败: {:?}", e),
    }
    
    // 从 UTF-8 字节创建（不安全版本）
    let bytes = vec![87, 111, 114, 108, 100]; // "World" 的字节
    let s4 = unsafe { String::from_utf8_unchecked(bytes) };
    println!("使用 unsafe 从字节数组: '{}'", s4);
    
    // 使用 push_str 和 push 构建字符串
    let mut s5 = String::new();
    s5.push_str("Hello");
    s5.push(' ');
    s5.push_str("World");
    println!("使用 push_str 和 push 构建: '{}'", s5);
    
    println!();
}

fn string_modification() {
    println!("=== String 的修改操作 ===");
    
    let mut s = String::from("Hello");
    println!("初始字符串: '{}'", s);
    
    // 追加字符串
    s.push_str(", World!");
    println!("push_str 后: '{}'", s);
    
    // 追加单个字符
    s.push('!');
    println!("push 后: '{}'", s);
    
    // 插入字符串到指定位置
    s.insert_str(5, " Beautiful");
    println!("insert_str 后: '{}'", s);
    
    // 插入单个字符到指定位置
    s.insert(0, '🎉');
    println!("insert 后: '{}'", s);
    
    // 移除字符
    s.remove(0); // 移除第一个字符（表情符号）
    println!("remove 后: '{}'", s);
    
    // 截断字符串
    s.truncate(13);
    println!("truncate 后: '{}'", s);
    
    // 清空字符串
    s.clear();
    println!("clear 后: '{}'（长度: {}）", s, s.len());
    
    // 使用 + 运算符连接字符串（需要拥有所有权）
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = s1 + &s2; // 注意：s1 被移动了，不能再使用
    println!("使用 + 运算符: '{}'", s3);
    println!("s2 仍然可用: '{}'", s2);
    
    // 使用 += 运算符
    let mut s4 = String::from("Rust");
    s4 += " is awesome!";
    println!("使用 += 运算符: '{}'", s4);
    
    // 替换字符串的一部分
    let mut s5 = String::from("I like Java");
    let range = 7..11; // "Java" 的范围
    s5.replace_range(range, "Rust");
    println!("replace_range 后: '{}'", s5);
    
    println!();
}

fn string_search_and_replace() {
    println!("=== String 的查找和替换 ===");
    
    let s = String::from("Hello, World! Welcome to the World of Rust!");
    println!("原始字符串: '{}'", s);
    
    // 查找子字符串
    if let Some(pos) = s.find("World") {
        println!("找到 'World' 在位置: {}", pos);
    }
    
    // 从指定位置开始查找
    if let Some(pos) = s[13..].find("World") {
        println!("从位置 13 开始找到 'World' 在相对位置: {}", pos);
    }
    
    // 查找字符
    if let Some(pos) = s.find('W') {
        println!("找到 'W' 在位置: {}", pos);
    }
    
    // 使用 contains 检查是否包含
    println!("是否包含 'Rust': {}", s.contains("Rust"));
    println!("是否包含 'Java': {}", s.contains("Java"));
    
    // 使用 starts_with 和 ends_with
    println!("是否以 'Hello' 开头: {}", s.starts_with("Hello"));
    println!("是否以 'Rust!' 结尾: {}", s.ends_with("Rust!"));
    
    // 替换所有匹配项
    let replaced = s.replace("World", "Universe");
    println!("替换所有 'World' 为 'Universe': '{}'", replaced);
    
    // 只替换第一个匹配项
    let replaced_once = s.replacen("World", "Universe", 1);
    println!("替换第一个 'World' 为 'Universe': '{}'", replaced_once);
    
    // 使用 match_indices 获取所有匹配位置
    println!("所有 'World' 的位置:");
    for (index, matched) in s.match_indices("World") {
        println!("  位置 {}: '{}'", index, matched);
    }
    
    // 使用 split 分割字符串
    let parts: Vec<&str> = s.split(' ').collect();
    println!("按空格分割: {:?}", parts);
    
    // 使用 split_whitespace 分割空白字符
    let words: Vec<&str> = s.split_whitespace().collect();
    println!("按空白字符分割: {:?}", words);
    
    // 使用 lines 分割行
    let multiline = String::from("Line 1\nLine 2\r\nLine 3");
    let lines: Vec<&str> = multiline.lines().collect();
    println!("多行字符串分割: {:?}", lines);
    
    println!();
}

fn string_split_and_join() {
    println!("=== String 的分割和连接 ===");
    
    // 分割字符串
    let csv_data = String::from("apple,banana,cherry,date");
    let fruits: Vec<&str> = csv_data.split(',').collect();
    println!("CSV 数据分割: {:?}", fruits);
    
    // 限制分割次数
    let limited_split: Vec<&str> = csv_data.splitn(2, ',').collect();
    println!("限制分割 2 次: {:?}", limited_split);
    
    // 从末尾开始分割
    let rsplit: Vec<&str> = csv_data.rsplitn(2, ',').collect();
    println!("从末尾分割 2 次: {:?}", rsplit);
    
    // 使用多个分隔符分割
    let mixed_data = String::from("apple,banana;cherry|date");
    let separators = &[',', ';', '|'];
    let mixed_split: Vec<&str> = mixed_data.split(|c| separators.contains(&c)).collect();
    println!("多分隔符分割: {:?}", mixed_split);
    
    // 连接字符串
    let words = vec!["Hello", "World", "from", "Rust"];
    let joined = words.join(" ");
    println!("使用 join 连接: '{}'", joined);
    
    // 使用 concat 连接
    let parts = vec!["Hello", ", ", "World", "!"];
    let concatenated: String = parts.concat();
    println!("使用 concat 连接: '{}'", concatenated);
    
    // 使用 format! 连接
    let name = "Alice";
    let age = 30;
    let city = "New York";
    let formatted = format!("Name: {}, Age: {}, City: {}", name, age, city);
    println!("使用 format! 连接: '{}'", formatted);
    
    // 构建复杂字符串
    let mut result = String::new();
    for i in 1..=5 {
        result.push_str(&format!("Item {}: value = {}\n", i, i * 10));
    }
    println!("构建复杂字符串:\n{}", result);
    
    // 使用 write! 宏（需要 std::fmt::Write trait）
    use std::fmt::Write;
    let mut buffer = String::new();
    writeln!(&mut buffer, "Line 1").unwrap();
    writeln!(&mut buffer, "Line 2").unwrap();
    writeln!(&mut buffer, "Line 3").unwrap();
    println!("使用 write! 宏构建:\n{}", buffer);
    
    println!();
}

fn string_conversions() {
    println!("=== String 的转换 ===");
    
    // String 转换为 &str
    let s = String::from("Hello, World!");
    let string_slice: &str = &s;
    println!("String 转换为 &str: '{}'", string_slice);
    
    // &str 转换为 String
    let str_slice = "Hello, Rust!";
    let owned_string = str_slice.to_string();
    println!("&str 转换为 String: '{}'", owned_string);
    
    // String 转换为字符数组
    let s = String::from("Hello");
    let chars: Vec<char> = s.chars().collect();
    println!("String 转换为字符数组: {:?}", chars);
    
    // 字符数组转换为 String
    let chars = vec!['R', 'u', 's', 't'];
    let string_from_chars: String = chars.iter().collect();
    println!("字符数组转换为 String: '{}'", string_from_chars);
    
    // String 转换为字节数组
    let s = String::from("Hello");
    let bytes = s.into_bytes();
    println!("String 转换为字节数组: {:?}", bytes);
    
    // 字节数组转换为 String
    let bytes = vec![72, 101, 108, 108, 111]; // "Hello"
    match String::from_utf8(bytes) {
        Ok(s) => println!("字节数组转换为 String: '{}'", s),
        Err(e) => println!("转换失败: {:?}", e),
    }
    
    // 转换为 ASCII 字符串（忽略非 ASCII 字符）
    let s = String::from("Hello 世界");
    let ascii_string = s.chars().filter(|c| c.is_ascii()).collect::<String>();
    println!("过滤 ASCII 字符: '{}'", ascii_string);
    
    // 大小写转换
    let s = String::from("Hello, World!");
    println!("原始字符串: '{}'", s);
    println!("转换为大写: '{}'", s.to_uppercase());
    println!("转换为小写: '{}'", s.to_lowercase());
    
    // 修剪空白字符
    let s_with_spaces = String::from("   Hello, World!   \n\t");
    println!("包含空白字符: '{}'", s_with_spaces);
    println!("修剪两侧空白: '{}'", s_with_spaces.trim());
    println!("修剪左侧空白: '{}'", s_with_spaces.trim_start());
    println!("修剪右侧空白: '{}'", s_with_spaces.trim_end());
    
    // 修剪特定字符
    let s_with_chars = String::from("***Hello, World!***");
    println!("包含特定字符: '{}'", s_with_chars);
    println!("修剪 '*' 字符: '{}'", s_with_chars.trim_matches('*'));
    
    // 数字转换
    let number_string = String::from("42");
    match number_string.parse::<i32>() {
        Ok(number) => println!("字符串转换为数字: {}", number),
        Err(e) => println!("转换失败: {:?}", e),
    }
    
    let float_string = String::from("3.14159");
    match float_string.parse::<f64>() {
        Ok(float) => println!("字符串转换为浮点数: {}", float),
        Err(e) => println!("转换失败: {:?}", e),
    }
    
    // 布尔值转换
    let bool_string = String::from("true");
    match bool_string.parse::<bool>() {
        Ok(boolean) => println!("字符串转换为布尔值: {}", boolean),
        Err(e) => println!("转换失败: {:?}", e),
    }
    
    println!();
}

fn advanced_string_usage() {
    println!("=== String 的高级用法 ===");
    
    // 1. 使用 String 作为缓冲区
    let mut buffer = String::new();
    for i in 1..=5 {
        buffer.push_str(&format!("Line {}: This is some text\n", i));
    }
    println!("使用 String 作为缓冲区:\n{}", buffer);
    
    // 2. 高效的字符串构建
    let mut efficient_string = String::with_capacity(100);
    for word in &["Hello", " ", "World", " ", "from", " ", "Rust"] {
        efficient_string.push_str(word);
    }
    println!("高效构建的字符串: '{}'", efficient_string);
    
    // 3. 字符串模式匹配
    let text = String::from("The quick brown fox jumps over the lazy dog");
    
    // 使用 match 进行模式匹配
    let result = match text.as_str() {
        s if s.starts_with("The") => "Starts with 'The'",
        s if s.contains("fox") => "Contains 'fox'",
        _ => "No match",
    };
    println!("模式匹配结果: '{}'", result);
    
    // 4. 字符串的内存管理
    let mut s = String::with_capacity(100);
    s.push_str("Hello");
    println!("容量: {}，长度: {}", s.capacity(), s.len());
    
    s.shrink_to_fit();
    println!("收缩后 - 容量: {}，长度: {}", s.capacity(), s.len());
    
    // 5. 使用 Cow（Clone on Write）进行高效字符串处理
    use std::borrow::Cow;
    
    fn process_string(input: &str) -> Cow<str> {
        if input.contains("hello") {
            Cow::Owned(input.replace("hello", "HELLO"))
        } else {
            Cow::Borrowed(input)
        }
    }
    
    let input1 = "hello world";
    let input2 = "goodbye world";
    
    match process_string(input1) {
        Cow::Borrowed(s) => println!("借用: '{}'", s),
        Cow::Owned(s) => println!("拥有: '{}'", s),
    }
    
    match process_string(input2) {
        Cow::Borrowed(s) => println!("借用: '{}'", s),
        Cow::Owned(s) => println!("拥有: '{}'", s),
    }
    
    // 6. 字符串的迭代处理
    let text = String::from("Hello, World! How are you?");
    let words: Vec<String> = text
        .split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
        .collect();
    println!("处理后的单词: {:?}", words);
    
    // 7. 正则表达式风格的简单匹配
    fn simple_pattern_match(text: &str, pattern: &str) -> bool {
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                return text.starts_with(parts[0]) && text.ends_with(parts[1]);
            }
        }
        text.contains(pattern)
    }
    
    println!("简单模式匹配测试:");
    println!("  'HelloWorld' contains 'Hello*World': {}", simple_pattern_match("HelloWorld", "Hello*World"));
    println!("  'HelloWorld' contains 'Hi*World': {}", simple_pattern_match("HelloWorld", "Hi*World"));
    
    // 8. 字符串的编码处理
    let emoji_string = String::from("Hello 👋 World 🌍!");
    println!("包含 emoji 的字符串: '{}'", emoji_string);
    println!("  字节长度: {}", emoji_string.len());
    println!("  字符数: {}", emoji_string.chars().count());
    
    // 9. 使用 String 构建复杂文本
    fn build_report(title: &str, items: &[(&str, i32)]) -> String {
        let mut report = String::new();
        report.push_str(&format!("=== {} ===\n", title));
        report.push_str(&format!("总项目数: {}\n\n", items.len()));
        
        for (name, value) in items {
            report.push_str(&format!("{}: {}\n", name, value));
        }
        
        report.push_str(&format!("\n总计: {}\n", items.iter().map(|(_, v)| v).sum::<i32>()));
        report
    }
    
    let items = vec![("Apples", 10), ("Bananas", 15), ("Cherries", 8)];
    let report = build_report("水果库存报告", &items);
    println!("构建的报告:\n{}", report);
    
    println!();
}
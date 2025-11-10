use std::collections::HashMap;

fn main() {
    println!("=== HashMap 学习示例 ===\n");

    // 基础 HashMap 使用
    basic_hashmap();
    
    // HashMap 的创建和初始化
    hashmap_creation();
    
    // HashMap 的元素操作
    hashmap_element_operations();
    
    // HashMap 的迭代
    hashmap_iteration();
    
    // HashMap 的高级用法
    advanced_hashmap_usage();
}

fn basic_hashmap() {
    println!("=== 基础 HashMap 使用 ===");
    
    // 创建空 HashMap
    let mut scores = HashMap::new();
    println!("空 HashMap: {:?}", scores);
    
    // 插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("插入后: {:?}", scores);
    
    // 获取值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue 队的得分: {}", score);
    
    // 检查是否包含键
    println!("是否包含 Red 队: {}", scores.contains_key("Red"));
    
    // 获取长度
    println!("HashMap 长度: {}", scores.len());
    
    println!();
}

fn hashmap_creation() {
    println!("=== HashMap 的创建和初始化 ===");
    
    // 从元组数组创建
    let teams = [("Blue".to_string(), 10), ("Yellow".to_string(), 50)];
    let scores: HashMap<_, _> = teams.into_iter().collect();
    println!("从元组数组创建: {:?}", scores);
    
    // 使用 with_capacity 创建
    let mut map = HashMap::with_capacity(10);
    map.insert("one", 1);
    println!("预分配容量的 HashMap: {:?}", map);
    
    println!();
}

fn hashmap_element_operations() {
    println!("=== HashMap 的元素操作 ===");
    
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    println!("初始 HashMap: {:?}", scores);
    
    // 更新值
    scores.insert("Blue", 25);
    println!("更新 Blue 队得分后: {:?}", scores);
    
    // 使用 entry API
    scores.entry("Yellow").or_insert(60);
    scores.entry("Red").or_insert(30);
    println!("使用 entry API 后: {:?}", scores);
    
    // 根据旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("单词计数: {:?}", map);
    
    // 移除键值对
    scores.remove("Yellow");
    println!("移除 Yellow 队后: {:?}", scores);
    
    // 清空 HashMap
    scores.clear();
    println!("清空后: {:?}", scores);
    
    println!();
}

fn hashmap_iteration() {
    println!("=== HashMap 的迭代 ===");
    
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    scores.insert("Red", 30);
    
    // 迭代键值对
    println!("迭代键值对:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }
    
    // 迭代键
    println!("\n迭代键:");
    for key in scores.keys() {
        println!("  {}", key);
    }
    
    // 迭代值
    println!("\n迭代值:");
    for value in scores.values() {
        println!("  {}", value);
    }
    
    // 可变迭代值
    println!("\n可变迭代值:");
    for value in scores.values_mut() {
        *value *= 2;
    }
    println!("修改后的 HashMap: {:?}", scores);
    
    println!();
}

fn advanced_hashmap_usage() {
    println!("=== HashMap 的高级用法 ===");
    
    // 1. 自定义哈希器
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct CustomKey {
        id: u32,
        name: String,
    }
    
    let mut map = HashMap::new();
    map.insert(CustomKey { id: 1, name: "one".to_string() }, 100);
    println!("使用自定义键的 HashMap: {:?}", map);
    
    // 2. 内存管理
    let mut map = HashMap::with_capacity(100);
    map.insert(1, 1);
    println!("初始容量: {}", map.capacity());
    
    map.shrink_to_fit();
    println!("收缩后的容量: {}", map.capacity());
    
    // 3. 复杂值的 HashMap
    let mut user_roles: HashMap<String, Vec<String>> = HashMap::new();
    user_roles.entry("Alice".to_string()).or_default().push("admin".to_string());
    user_roles.entry("Bob".to_string()).or_default().push("user".to_string());
    user_roles.entry("Alice".to_string()).or_default().push("auditor".to_string());
    println!("用户角色: {:?}", user_roles);
    
    println!();
}
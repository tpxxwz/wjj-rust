use std::collections::BTreeMap;

fn main() {
    println!("=== BTreeMap 学习示例 ===\n");

    // 基础 BTreeMap 使用
    basic_btreemap();
    
    // BTreeMap 的创建和初始化
    btreemap_creation();
    
    // BTreeMap 的元素操作
    btreemap_element_operations();
    
    // BTreeMap 的迭代
    btreemap_iteration();
    
    // BTreeMap 的高级用法
    advanced_btreemap_usage();
}

fn basic_btreemap() {
    println!("=== 基础 BTreeMap 使用 ===");
    
    // 创建空 BTreeMap
    let mut map = BTreeMap::new();
    println!("空 BTreeMap: {:?}", map);
    
    // 插入键值对
    map.insert(3, "c");
    map.insert(1, "a");
    map.insert(2, "b");
    println!("插入后 (自动排序): {:?}", map);
    
    // 获取值
    let value = map.get(&1).unwrap();
    println!("键 1 的值: {}", value);
    
    // 检查是否包含键
    println!("是否包含键 4: {}", map.contains_key(&4));
    
    // 获取长度
    println!("BTreeMap 长度: {}", map.len());
    
    println!();
}

fn btreemap_creation() {
    println!("=== BTreeMap 的创建和初始化 ===");
    
    // 从元组数组创建
    let data = [(3, "c"), (1, "a"), (2, "b")];
    let map: BTreeMap<_, _> = data.into_iter().collect();
    println!("从元组数组创建: {:?}", map);
    
    println!();
}

fn btreemap_element_operations() {
    println!("=== BTreeMap 的元素操作 ===");
    
    let mut map = BTreeMap::new();
    map.insert(2, "b");
    map.insert(1, "a");
    map.insert(3, "c");
    println!("初始 BTreeMap: {:?}", map);
    
    // 更新值
    map.insert(2, "beta");
    println!("更新键 2 的值后: {:?}", map);
    
    // 使用 entry API
    map.entry(4).or_insert("d");
    map.entry(1).or_insert("alpha");
    println!("使用 entry API 后: {:?}", map);
    
    // 移除键值对
    map.remove(&2);
    println!("移除键 2 后: {:?}", map);
    
    // 获取并移除第一个和最后一个元素
    if let Some((key, value)) = map.pop_first() {
        println!("移除的第一个元素: ({}, {})", key, value);
    }
    if let Some((key, value)) = map.pop_last() {
        println!("移除的最后一个元素: ({}, {})", key, value);
    }
    println!("移除第一个和最后一个元素后: {:?}", map);
    
    // 清空 BTreeMap
    map.clear();
    println!("清空后: {:?}", map);
    
    println!();
}

fn btreemap_iteration() {
    println!("=== BTreeMap 的迭代 ===");
    
    let mut map = BTreeMap::new();
    map.insert(3, "c");
    map.insert(1, "a");
    map.insert(2, "b");
    
    // 迭代键值对 (按键排序)
    println!("迭代键值对:");
    for (key, value) in &map {
        println!("  {}: {}", key, value);
    }
    
    // 迭代键
    println!("\n迭代键:");
    for key in map.keys() {
        println!("  {}", key);
    }
    
    // 迭代值
    println!("\n迭代值:");
    for value in map.values() {
        println!("  {}", value);
    }
    
    // 可变迭代值
    println!("\n可变迭代值:");
    for value in map.values_mut() {
        *value = "modified";
    }
    println!("修改后的 BTreeMap: {:?}", map);
    
    // 反向迭代
    println!("\n反向迭代:");
    for (key, value) in map.iter().rev() {
        println!("  {}: {}", key, value);
    }
    
    println!();
}

fn advanced_btreemap_usage() {
    println!("=== BTreeMap 的高级用法 ===");
    
    let mut map = BTreeMap::new();
    map.insert(10, "a");
    map.insert(20, "b");
    map.insert(30, "c");
    map.insert(40, "d");
    map.insert(50, "e");
    
    // 1. 范围查询
    println!("范围查询 (20..=40):");
    for (key, value) in map.range(20..=40) {
        println!("  {}: {}", key, value);
    }
    
    // 2. 可变范围查询
    println!("\n可变范围查询 (30..):");
    for (_, value) in map.range_mut(30..) {
        *value = "changed";
    }
    println!("修改后的 BTreeMap: {:?}", map);
    
    // 3. 分割 BTreeMap
    let right_half = map.split_off(&30);
    println!("\n分割后 - 左半部分: {:?}", map);
    println!("分割后 - 右半部分: {:?}", right_half);
    
    // 4. 追加 BTreeMap
    map.append(&mut BTreeMap::from([(25, "z"), (35, "y")]));
    println!("\n追加后: {:?}", map);
    
    println!();
}
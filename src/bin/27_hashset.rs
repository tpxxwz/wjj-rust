use std::collections::HashSet;

fn main() {
    println!("=== HashSet 学习示例 ===\n");

    // 基础 HashSet 使用
    basic_hashset();
    
    // HashSet 的创建和初始化
    hashset_creation();
    
    // HashSet 的元素操作
    hashset_element_operations();
    
    // HashSet 的集合操作
    hashset_set_operations();
    
    // HashSet 的迭代
    hashset_iteration();
}

fn basic_hashset() {
    println!("=== 基础 HashSet 使用 ===");
    
    // 创建空 HashSet
    let mut set = HashSet::new();
    println!("空 HashSet: {:?}", set);
    
    // 插入元素
    set.insert("a");
    set.insert("b");
    set.insert("a"); // 重复元素无效
    println!("插入后: {:?}", set);
    
    // 检查是否包含元素
    println!("是否包含 'a': {}", set.contains("a"));
    println!("是否包含 'c': {}", set.contains("c"));
    
    // 获取长度
    println!("HashSet 长度: {}", set.len());
    
    println!();
}

fn hashset_creation() {
    println!("=== HashSet 的创建和初始化 ===");
    
    // 从数组创建
    let data = [1, 2, 3, 2, 1];
    let set: HashSet<_> = data.into_iter().collect();
    println!("从数组创建: {:?}", set);
    
    // 使用 with_capacity 创建
    let mut set = HashSet::with_capacity(10);
    set.insert(1);
    println!("预分配容量的 HashSet: {:?}", set);
    
    println!();
}

fn hashset_element_operations() {
    println!("=== HashSet 的元素操作 ===");
    
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("初始 HashSet: {:?}", set);
    
    // 移除元素
    set.remove(&2);
    println!("移除 2 后: {:?}", set);
    
    // 清空 HashSet
    set.clear();
    println!("清空后: {:?}", set);
    
    println!();
}

fn hashset_set_operations() {
    println!("=== HashSet 的集合操作 ===");
    
    let set1: HashSet<_> = [1, 2, 3, 4].iter().collect();
    let set2: HashSet<_> = [3, 4, 5, 6].iter().collect();
    println!("Set1: {:?}", set1);
    println!("Set2: {:?}", set2);
    
    // 并集
    let union: HashSet<_> = set1.union(&set2).collect();
    println!("并集: {:?}", union);
    
    // 交集
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("交集: {:?}", intersection);
    
    // 差集
    let difference: HashSet<_> = set1.difference(&set2).collect();
    println!("差集 (Set1 - Set2): {:?}", difference);
    
    // 对称差集
    let symmetric_difference: HashSet<_> = set1.symmetric_difference(&set2).collect();
    println!("对称差集: {:?}", symmetric_difference);
    
    // 子集和超集
    let subset: HashSet<_> = [1, 2].iter().collect();
    println!("Set1 是否是 [1, 2] 的超集: {}", set1.is_superset(&subset));
    println!("[1, 2] 是否是 Set1 的子集: {}", subset.is_subset(&set1));
    
    println!();
}

fn hashset_iteration() {
    println!("=== HashSet 的迭代 ===");
    
    let mut set = HashSet::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");
    
    // 迭代元素 (无序)
    println!("迭代元素:");
    for value in &set {
        println!("  {}", value);
    }
    
    println!();
}
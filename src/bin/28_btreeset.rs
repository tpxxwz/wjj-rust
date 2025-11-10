use std::collections::BTreeSet;

fn main() {
    println!("=== BTreeSet 学习示例 ===\n");

    // 基础 BTreeSet 使用
    basic_btreeset();
    
    // BTreeSet 的创建和初始化
    btreeset_creation();
    
    // BTreeSet 的元素操作
    btreeset_element_operations();
    
    // BTreeSet 的集合操作
    btreeset_set_operations();
    
    // BTreeSet 的迭代
    btreeset_iteration();
}

fn basic_btreeset() {
    println!("=== 基础 BTreeSet 使用 ===");
    
    // 创建空 BTreeSet
    let mut set = BTreeSet::new();
    println!("空 BTreeSet: {:?}", set);
    
    // 插入元素
    set.insert(3);
    set.insert(1);
    set.insert(2);
    set.insert(1); // 重复元素无效
    println!("插入后 (自动排序): {:?}", set);
    
    // 检查是否包含元素
    println!("是否包含 2: {}", set.contains(&2));
    println!("是否包含 4: {}", set.contains(&4));
    
    // 获取长度
    println!("BTreeSet 长度: {}", set.len());
    
    println!();
}

fn btreeset_creation() {
    println!("=== BTreeSet 的创建和初始化 ===");
    
    // 从数组创建
    let data = [3, 1, 2, 1, 3];
    let set: BTreeSet<_> = data.into_iter().collect();
    println!("从数组创建: {:?}", set);
    
    println!();
}

fn btreeset_element_operations() {
    println!("=== BTreeSet 的元素操作 ===");
    
    let mut set = BTreeSet::new();
    set.insert(2);
    set.insert(1);
    set.insert(3);
    println!("初始 BTreeSet: {:?}", set);
    
    // 移除元素
    set.remove(&2);
    println!("移除 2 后: {:?}", set);
    
    // 获取并移除第一个和最后一个元素
    if let Some(first) = set.pop_first() {
        println!("移除的第一个元素: {}", first);
    }
    if let Some(last) = set.pop_last() {
        println!("移除的最后一个元素: {}", last);
    }
    println!("移除第一个和最后一个元素后: {:?}", set);
    
    // 清空 BTreeSet
    set.clear();
    println!("清空后: {:?}", set);
    
    println!();
}

fn btreeset_set_operations() {
    println!("=== BTreeSet 的集合操作 ===");
    
    let set1: BTreeSet<_> = [1, 2, 3, 4].iter().collect();
    let set2: BTreeSet<_> = [3, 4, 5, 6].iter().collect();
    println!("Set1: {:?}", set1);
    println!("Set2: {:?}", set2);
    
    // 并集
    let union: BTreeSet<_> = set1.union(&set2).collect();
    println!("并集: {:?}", union);
    
    // 交集
    let intersection: BTreeSet<_> = set1.intersection(&set2).collect();
    println!("交集: {:?}", intersection);
    
    // 差集
    let difference: BTreeSet<_> = set1.difference(&set2).collect();
    println!("差集 (Set1 - Set2): {:?}", difference);
    
    // 对称差集
    let symmetric_difference: BTreeSet<_> = set1.symmetric_difference(&set2).collect();
    println!("对称差集: {:?}", symmetric_difference);
    
    // 子集和超集
    let subset: BTreeSet<_> = [1, 2].iter().collect();
    println!("Set1 是否是 [1, 2] 的超集: {}", set1.is_superset(&subset));
    println!("[1, 2] 是否是 Set1 的子集: {}", subset.is_subset(&set1));
    
    println!();
}

fn btreeset_iteration() {
    println!("=== BTreeSet 的迭代 ===");
    
    let mut set = BTreeSet::new();
    set.insert(3);
    set.insert(1);
    set.insert(2);
    
    // 迭代元素 (按键排序)
    println!("迭代元素:");
    for value in &set {
        println!("  {}", value);
    }
    
    // 反向迭代
    println!("\n反向迭代:");
    for value in set.iter().rev() {
        println!("  {}", value);
    }
    
    // 范围查询
    println!("\n范围查询 (2..=3):");
    for value in set.range(2..=3) {
        println!("  {}", value);
    }
    
    println!();
}
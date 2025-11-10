fn main() {
    println!("=== Vec 学习示例 ===\n");

    // 基础 Vec 使用
    basic_vec();
    
    // Vec 的创建和初始化
    vec_creation();
    
    // Vec 的元素操作
    vec_element_operations();
    
    // Vec 的迭代
    vec_iteration();
    
    // Vec 的排序和搜索
    vec_sorting_and_searching();
    
    // Vec 的高级用法
    advanced_vec_usage();
}

fn basic_vec() {
    println!("=== 基础 Vec 使用 ===");
    
    // 创建空 Vec
    let mut vec1: Vec<i32> = Vec::new();
    println!("空 Vec: {:?}", vec1);
    
    // 使用宏创建 Vec
    let vec2 = vec![1, 2, 3, 4, 5];
    println!("使用 vec! 宏创建: {:?}", vec2);
    
    // 创建指定容量的 Vec
    let mut vec3 = Vec::with_capacity(10);
    println!("容量为 10 的 Vec，长度: {}", vec3.len());
    
    // 添加元素
    vec1.push(10);
    vec1.push(20);
    vec1.push(30);
    println!("添加元素后: {:?}", vec1);
    
    // 获取长度和容量
    println!("Vec1 长度: {}, 容量: {}", vec1.len(), vec1.capacity());
    
    // 检查是否为空
    println!("Vec1 是否为空: {}", vec1.is_empty());
    
    println!();
}

fn vec_creation() {
    println!("=== Vec 的创建和初始化 ===");
    
    // 从数组创建
    let arr = [1, 2, 3, 4, 5];
    let vec_from_arr = Vec::from(arr);
    println!("从数组创建: {:?}", vec_from_arr);
    
    // 从切片创建
    let slice = &[10, 20, 30];
    let vec_from_slice = slice.to_vec();
    println!("从切片创建: {:?}", vec_from_slice);
    
    // 使用迭代器创建
    let vec_from_iter: Vec<i32> = (0..5).collect();
    println!("使用迭代器创建: {:?}", vec_from_iter);
    
    // 重复元素创建
    let vec_repeat = vec![42; 5];
    println!("重复元素创建: {:?}", vec_repeat);
    
    // 使用 Vec::new() 并添加元素
    let mut vec = Vec::new();
    vec.extend([1, 2, 3].iter());
    println!("使用 extend 添加元素: {:?}", vec);
    
    // 使用宏创建不同类型的 Vec
    let str_vec = vec!["hello", "world", "rust"];
    let char_vec = vec!['a', 'b', 'c'];
    println!("字符串 Vec: {:?}", str_vec);
    println!("字符 Vec: {:?}", char_vec);
    
    println!();
}

fn vec_element_operations() {
    println!("=== Vec 的元素操作 ===");
    
    let mut vec = vec![10, 20, 30, 40, 50];
    println!("初始 Vec: {:?}", vec);
    
    // 索引访问
    println!("第一个元素: {}", vec[0]);
    println!("第三个元素: {}", vec[2]);
    
    // 安全索引访问
    match vec.get(2) {
        Some(value) => println!("get(2): {}", value),
        None => println!("索引 2 越界"),
    }
    
    match vec.get(10) {
        Some(value) => println!("get(10): {}", value),
        None => println!("索引 10 越界"),
    }
    
    // 修改元素
    vec[1] = 25;
    println!("修改索引 1 后: {:?}", vec);
    
    // 弹出最后一个元素
    if let Some(last) = vec.pop() {
        println!("弹出的最后一个元素: {}", last);
        println!("弹出后: {:?}", vec);
    }
    
    // 移除指定位置的元素
    let removed = vec.remove(1);
    println!("移除索引 1 的元素: {}", removed);
    println!("移除后: {:?}", vec);
    
    // 在指定位置插入元素
    vec.insert(1, 35);
    println!("在索引 1 插入 35: {:?}", vec);
    
    // 获取 Vec 的最后一个元素
    if let Some(last) = vec.last() {
        println!("最后一个元素: {}", last);
    }
    
    // 获取 Vec 的第一个元素
    if let Some(first) = vec.first() {
        println!("第一个元素: {}", first);
    }
    
    // 清空 Vec
    vec.clear();
    println!("清空后: {:?}，长度: {}", vec, vec.len());
    
    println!();
}

fn vec_iteration() {
    println!("=== Vec 的迭代 ===");
    
    let vec = vec![1, 2, 3, 4, 5];
    
    // 不可变迭代
    println!("不可变迭代:");
    for (index, value) in vec.iter().enumerate() {
        println!("  索引 {}: {}", index, value);
    }
    
    // 可变迭代
    let mut vec_mut = vec![10, 20, 30, 40, 50];
    println!("\n可变迭代:");
    for value in vec_mut.iter_mut() {
        *value *= 2;
    }
    println!("修改后的 Vec: {:?}", vec_mut);
    
    // 消费迭代（获取所有权）
    let vec_own = vec!["hello", "world", "rust"];
    println!("\n消费迭代:");
    for string in vec_own.into_iter() {
        println!("  {}", string);
    }
    // vec_own 已经被消费，不能再使用
    
    // 使用迭代器方法
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("\n使用 map 和 collect: {:?}", doubled);
    
    // 过滤元素
    let filtered: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("过滤偶数: {:?}", filtered);
    
    // 枚举迭代
    let enumerated: Vec<(usize, i32)> = numbers.iter().enumerate().map(|(i, &x)| (i, x)).collect();
    println!("枚举迭代: {:?}", enumerated);
    
    // 反向迭代
    let reversed: Vec<i32> = numbers.iter().rev().copied().collect();
    println!("反向迭代: {:?}", reversed);
    
    println!();
}

fn vec_sorting_and_searching() {
    println!("=== Vec 的排序和搜索 ===");
    
    // 排序
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("排序前: {:?}", numbers);
    
    numbers.sort();
    println!("排序后: {:?}", numbers);
    
    // 自定义排序
    let mut words = vec!["banana", "apple", "cherry", "date"];
    words.sort_by(|a, b| b.len().cmp(&a.len())); // 按长度降序排序
    println!("按长度降序排序: {:?}", words);
    
    // 二分查找（要求 Vec 已排序）
    let sorted_numbers = vec![1, 3, 5, 7, 9, 11, 13, 15];
    match sorted_numbers.binary_search(&7) {
        Ok(index) => println!("找到 7 在索引: {}", index),
        Err(insert_index) => println!("未找到 7，可插入索引: {}", insert_index),
    }
    
    match sorted_numbers.binary_search(&6) {
        Ok(index) => println!("找到 6 在索引: {}", index),
        Err(insert_index) => println!("未找到 6，可插入索引: {}", insert_index),
    }
    
    // 查找元素
    let fruits = vec!["apple", "banana", "cherry", "apple"];
    if let Some(index) = fruits.iter().position(|&x| x == "banana") {
        println!("找到 banana 在索引: {}", index);
    }
    
    // 查找所有匹配的元素
    let apple_positions: Vec<usize> = fruits.iter()
        .enumerate()
        .filter(|(_, &x)| x == "apple")
        .map(|(i, _)| i)
        .collect();
    println!("所有 apple 的位置: {:?}", apple_positions);
    
    // 最大值和最小值
    if let Some(max) = numbers.iter().max() {
        println!("最大值: {}", max);
    }
    
    if let Some(min) = numbers.iter().min() {
        println!("最小值: {}", min);
    }
    
    println!();
}

fn advanced_vec_usage() {
    println!("=== Vec 的高级用法 ===");
    
    // 1. Vec 的分割
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let split_index = 4;
    let right_half = vec.split_off(split_index);
    println!("分割后 - 左半部分: {:?}", vec);
    println!("分割后 - 右半部分: {:?}", right_half);
    
    // 2. Vec 的连接
    let mut vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    vec1.extend(vec2);
    println!("连接后的 Vec: {:?}", vec1);
    
    // 3. 使用 drain 移除元素
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let drained: Vec<i32> = vec.drain(2..5).collect();
    println!("drain 移除的元素: {:?}", drained);
    println!("drain 后的 Vec: {:?}", vec);
    
    // 4. 保留符合条件的元素
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    vec.retain(|&x| x % 2 == 0);
    println!("保留偶数后的 Vec: {:?}", vec);
    
    // 5. 使用 Windows
    let numbers = vec![1, 2, 3, 4, 5];
    println!("使用 windows(2):");
    for window in numbers.windows(2) {
        println!("  {:?}", window);
    }
    
    // 6. 使用 Chunks
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("使用 chunks(3):");
    for chunk in data.chunks(3) {
        println!("  {:?}", chunk);
    }
    
    // 7. Vec 的内存管理
    let mut vec = Vec::with_capacity(100);
    println!("初始容量: {}", vec.capacity());
    
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("添加元素后的容量: {}", vec.capacity());
    
    vec.shrink_to_fit();
    println!("收缩后的容量: {}", vec.capacity());
    
    // 8. 使用 Vec 作为栈
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("栈: {:?}", stack);
    
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
    
    // 9. 使用 Vec 作为队列（效率不高，仅作示例）
    let mut queue = Vec::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    println!("队列: {:?}", queue);
    
    while !queue.is_empty() {
        let front = queue.remove(0); // 注意：这是 O(n) 操作
        println!("出队: {}", front);
    }
    
    // 10. 自定义类型的 Vec
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let mut people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    // 按年龄排序
    people.sort_by(|a, b| a.age.cmp(&b.age));
    println!("按年龄排序后: {:?}", people);
    
    // 查找特定年龄的人
    if let Some(person) = people.iter().find(|p| p.age == 30) {
        println!("找到年龄为 30 的人: {:?}", person);
    }
    
    println!();
}
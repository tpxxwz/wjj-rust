use std::collections::VecDeque;

fn main() {
    println!("=== VecDeque 学习示例 ===\n");

    // 基础 VecDeque 使用
    basic_vecdeque();
    
    // VecDeque 的创建和初始化
    vecdeque_creation();
    
    // VecDeque 的元素操作
    vecdeque_element_operations();
    
    // VecDeque 的迭代
    vecdeque_iteration();
    
    // VecDeque 的高级用法
    advanced_vecdeque_usage();
}

fn basic_vecdeque() {
    println!("=== 基础 VecDeque 使用 ===");
    
    // 创建空 VecDeque
    let mut deque: VecDeque<i32> = VecDeque::new();
    println!("空 VecDeque: {:?}", deque);
    
    // 在队尾添加元素
    deque.push_back(10);
    deque.push_back(20);
    deque.push_back(30);
    println!("在队尾添加元素后: {:?}", deque);
    
    // 在队头添加元素
    deque.push_front(5);
    deque.push_front(0);
    println!("在队头添加元素后: {:?}", deque);
    
    // 获取长度和容量
    println!("VecDeque 长度: {}, 容量: {}", deque.len(), deque.capacity());
    
    // 检查是否为空
    println!("VecDeque 是否为空: {}", deque.is_empty());
    
    println!();
}

fn vecdeque_creation() {
    println!("=== VecDeque 的创建和初始化 ===");
    
    // 从 Vec 创建
    let vec = vec![1, 2, 3, 4, 5];
    let deque_from_vec: VecDeque<_> = vec.into();
    println!("从 Vec 创建: {:?}", deque_from_vec);
    
    // 使用迭代器创建
    let deque_from_iter: VecDeque<i32> = (0..5).collect();
    println!("使用迭代器创建: {:?}", deque_from_iter);
    
    // 使用 with_capacity 创建
    let mut deque = VecDeque::with_capacity(10);
    deque.push_back(1);
    println!("预分配容量的 VecDeque: {:?}", deque);
    
    println!();
}

fn vecdeque_element_operations() {
    println!("=== VecDeque 的元素操作 ===");
    
    let mut deque = VecDeque::from(vec![10, 20, 30, 40, 50]);
    println!("初始 VecDeque: {:?}", deque);
    
    // 索引访问
    println!("第一个元素: {}", deque[0]);
    println!("第三个元素: {}", deque[2]);
    
    // 安全索引访问
    match deque.get(2) {
        Some(value) => println!("get(2): {}", value),
        None => println!("索引 2 越界"),
    }
    
    // 修改元素
    if let Some(elem) = deque.get_mut(1) {
        *elem = 25;
    }
    println!("修改索引 1 后: {:?}", deque);
    
    // 从队头移除元素
    if let Some(front) = deque.pop_front() {
        println!("从队头移除的元素: {}", front);
        println!("移除后: {:?}", deque);
    }
    
    // 从队尾移除元素
    if let Some(back) = deque.pop_back() {
        println!("从队尾移除的元素: {}", back);
        println!("移除后: {:?}", deque);
    }
    
    // 在指定位置插入元素
    deque.insert(1, 35);
    println!("在索引 1 插入 35: {:?}", deque);
    
    // 移除指定位置的元素
    if let Some(removed) = deque.remove(2) {
        println!("移除索引 2 的元素: {}", removed);
        println!("移除后: {:?}", deque);
    }
    
    // 获取队头和队尾元素
    println!("队头元素: {:?}", deque.front());
    println!("队尾元素: {:?}", deque.back());
    
    // 清空 VecDeque
    deque.clear();
    println!("清空后: {:?}，长度: {}", deque, deque.len());
    
    println!();
}

fn vecdeque_iteration() {
    println!("=== VecDeque 的迭代 ===");
    
    let deque = VecDeque::from(vec![1, 2, 3, 4, 5]);
    
    // 不可变迭代
    println!("不可变迭代:");
    for (index, value) in deque.iter().enumerate() {
        println!("  索引 {}: {}", index, value);
    }
    
    // 可变迭代
    let mut deque_mut = VecDeque::from(vec![10, 20, 30, 40, 50]);
    println!("\n可变迭代:");
    for value in deque_mut.iter_mut() {
        *value *= 2;
    }
    println!("修改后的 VecDeque: {:?}", deque_mut);
    
    // 消费迭代
    let deque_own = VecDeque::from(vec!["hello", "world", "rust"]);
    println!("\n消费迭代:");
    for string in deque_own.into_iter() {
        println!("  {}", string);
    }
    
    println!();
}

fn advanced_vecdeque_usage() {
    println!("=== VecDeque 的高级用法 ===");
    
    // 1. 环形缓冲区
    let mut buffer = VecDeque::with_capacity(3);
    println!("容量为 3 的环形缓冲区");
    
    buffer.push_back(1);
    buffer.push_back(2);
    buffer.push_back(3);
    println!("初始缓冲区: {:?}", buffer);
    
    buffer.push_back(4); // 此时会自动扩容
    let _ = buffer.pop_front();
    println!("移除队头并添加新元素后: {:?}", buffer);
    
    // 2. 分割 VecDeque
    let mut deque = VecDeque::from(vec![1, 2, 3, 4, 5, 6]);
    let right_half = deque.split_off(3);
    println!("分割后 - 左半部分: {:?}", deque);
    println!("分割后 - 右半部分: {:?}", right_half);
    
    // 3. 旋转元素
    let mut deque = VecDeque::from(vec![1, 2, 3, 4, 5]);
    deque.rotate_left(2);
    println!("向左旋转 2 个位置: {:?}", deque);
    
    deque.rotate_right(3);
    println!("向右旋转 3 个位置: {:?}", deque);
    
    // 4. 获取连续的切片
    let mut deque = VecDeque::from(vec![1, 2, 3, 4, 5]);
    deque.push_front(0);
    deque.pop_back();
    
    let (slice1, slice2) = deque.as_slices();
    println!("连续切片: {:?} 和 {:?}", slice1, slice2);
    
    // 5. 交换元素
    let mut deque = VecDeque::from(vec![1, 2, 3, 4, 5]);
    deque.swap(0, 4);
    println!("交换索引 0 和 4 的元素后: {:?}", deque);
    
    println!();
}
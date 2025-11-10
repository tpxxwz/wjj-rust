fn main() {
    println!("=== Iterator Trait 学习示例 ===\n");

    // 1. 迭代器基础
    iterator_basics();

    // 2. 消耗型适配器 (Consuming Adaptors)
    consuming_adaptors();

    // 3. 迭代器适配器 (Iterator Adaptors)
    iterator_adaptors();

    // 4. 自定义迭代器
    custom_iterator();

    // 5. `into_iter`, `iter`, `iter_mut`
    iter_types();
}

fn iterator_basics() {
    println!("--- 1. 迭代器基础 ---");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // 创建一个迭代器

    // 使用 for 循环遍历迭代器
    print!("Using for loop: ");
    for val in v1_iter {
        print!("{}, ", val);
    }
    println!();

    // 手动调用 next() 方法
    let mut v1_iter_manual = v1.iter();
    assert_eq!(v1_iter_manual.next(), Some(&1));
    assert_eq!(v1_iter_manual.next(), Some(&2));
    assert_eq!(v1_iter_manual.next(), Some(&3));
    assert_eq!(v1_iter_manual.next(), None);
    println!("Manually called next() successfully.");

    println!();
}

fn consuming_adaptors() {
    println!("--- 2. 消耗型适配器 (Consuming Adaptors) ---");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // `sum()` 是一个消耗型适配器，它会消耗迭代器
    let total: i32 = v1_iter.sum();
    println!("Sum of elements: {}", total);

    // `v1_iter` 已经被消耗，不能再次使用
    // for val in v1_iter { // 这行代码会编译错误
    //     println!("This will not compile.");
    // }

    println!();
}

fn iterator_adaptors() {
    println!("--- 3. 迭代器适配器 (Iterator Adaptors) ---");

    let v1: Vec<i32> = vec![1, 2, 3];

    // `map()` 是一个迭代器适配器，它会创建一个新的迭代器
    let mapped_iter = v1.iter().map(|x| x * 2);
    println!("Mapped iterator (lazy, not yet consumed)");

    // `collect()` 是一个消耗型适配器，它会收集结果到一个集合中
    let v2: Vec<i32> = mapped_iter.collect();
    println!("Collected results after map: {:?}", v2);

    // 链式调用多个适配器
    let v3: Vec<i32> = v1.iter()
        .map(|x| x + 1)
        .filter(|&x| x > 2)
        .collect();
    println!("Chained adaptors (map and filter): {:?}", v3);

    println!();
}

// 自定义一个计数器结构体
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

// 为 Counter 实现 Iterator trait
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn custom_iterator() {
    println!("--- 4. 自定义迭代器 ---");

    let counter = Counter::new(5);

    print!("Custom counter: ");
    for i in counter {
        print!("{}, ", i);
    }
    println!();

    // 使用其他迭代器方法
    let sum_of_counter: u32 = Counter::new(5).sum();
    println!("Sum of custom counter: {}", sum_of_counter);

    let squared_counter: Vec<u32> = Counter::new(5)
        .map(|x| x * x)
        .collect();
    println!("Squared custom counter: {:?}", squared_counter);

    println!();
}

fn iter_types() {
    println!("--- 5. `into_iter`, `iter`, `iter_mut` ---");

    // `iter()` - 迭代不可变引用
    let v_immut = vec!['a', 'b', 'c'];
    print!("iter() (immutable references): ");
    for c in v_immut.iter() {
        print!("{}, ", c);
    }
    println!();
    println!("Original vector after iter(): {:?}", v_immut);

    // `into_iter()` - 迭代并获取所有权
    let v_owned = vec![10, 20, 30];
    print!("into_iter() (takes ownership): ");
    for i in v_owned.into_iter() {
        print!("{}, ", i);
    }
    println!();
    // `v_owned` 已经被移动，不能再使用
    // println!("Original vector after into_iter(): {:?}", v_owned); // 这行代码会编译错误

    // `iter_mut()` - 迭代可变引用
    let mut v_mut = vec![100, 200, 300];
    print!("iter_mut() (mutable references): ");
    for i in v_mut.iter_mut() {
        *i += 1;
        print!("{}, ", i);
    }
    println!();
    println!("Original vector after iter_mut(): {:?}", v_mut);

    println!();
}
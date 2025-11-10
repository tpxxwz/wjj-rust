use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("=== RefCell 学习示例 ===\n");

    // 基础 RefCell 使用
    basic_refcell();
    
    // RefCell 与 Rc 结合使用
    refcell_with_rc();
    
    // 动态借用检查
    dynamic_borrow_check();
    
    // 内部可变性模式
    interior_mutability_pattern();
    
    // 循环引用示例
    cyclic_reference_example();
    
    // 高级用法
    advanced_usage();
}

fn basic_refcell() {
    println!("=== 基础 RefCell 使用 ===");
    
    // 创建 RefCell
    let cell = RefCell::new(42);
    
    // 读取值
    println!("初始值: {}", cell.borrow());
    
    // 修改值
    *cell.borrow_mut() += 10;
    println!("修改后: {}", cell.borrow());
    
    // 多次不可变借用
    let val1 = cell.borrow();
    let val2 = cell.borrow();
    println!("两个不可变借用: {} 和 {}", val1, val2);
    drop(val1);
    drop(val2);
    
    // 可变借用
    let mut val = cell.borrow_mut();
    *val += 5;
    println!("可变借用修改: {}", val);
    drop(val);
    
    println!();
}

fn refcell_with_rc() {
    println!("=== RefCell 与 Rc 结合使用 ===");
    
    // 创建共享的可变数据
    let shared_data: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![1, 2, 3]));
    
    // 创建多个所有者
    let data1 = Rc::clone(&shared_data);
    let data2 = Rc::clone(&shared_data);
    
    // 通过 data1 修改数据
    data1.borrow_mut().push(4);
    println!("data1 修改后: {:?}", data1.borrow());
    
    // 通过 data2 读取数据
    println!("data2 读取: {:?}", data2.borrow());
    
    // 通过 data2 修改数据
    data2.borrow_mut().push(5);
    
    // 通过 data1 读取修改后的数据
    println!("最终数据: {:?}", data1.borrow());
    
    println!();
}

fn dynamic_borrow_check() {
    println!("=== 动态借用检查 ===");
    
    let cell = RefCell::new(vec![1, 2, 3]);
    
    // 正常借用
    {
        let b1 = cell.borrow();
        let b2 = cell.borrow();
        println!("两个不可变借用: {:?} 和 {:?}", b1, b2);
    }
    
    // 可变借用
    {
        let mut b = cell.borrow_mut();
        b.push(4);
        println!("可变借用修改: {:?}", b);
    }
    
    // 尝试在不可变借用存在时进行可变借用（会panic）
    // let b1 = cell.borrow();
    // let b2 = cell.borrow_mut(); // 这会导致panic
    
    // 使用 try_borrow 避免panic
    match cell.try_borrow() {
        Ok(val) => println!("成功不可变借用: {:?}", val),
        Err(e) => println!("不可变借用失败: {:?}", e),
    }
    
    match cell.try_borrow_mut() {
        Ok(mut val) => {
            val.push(5);
            println!("成功可变借用: {:?}", val);
        }
        Err(e) => println!("可变借用失败: {:?}", e),
    }
    
    println!();
}

fn interior_mutability_pattern() {
    println!("=== 内部可变性模式 ===");
    
    struct Counter {
        value: RefCell<i32>,
    }
    
    impl Counter {
        fn new() -> Self {
            Counter {
                value: RefCell::new(0),
            }
        }
        
        fn increment(&self) {
            *self.value.borrow_mut() += 1;
        }
        
        fn get(&self) -> i32 {
            *self.value.borrow()
        }
    }
    
    let counter = Counter::new();
    
    // 尽管 counter 是不可变的，但我们可以修改其内部状态
    counter.increment();
    counter.increment();
    counter.increment();
    
    println!("计数器值: {}", counter.get());
    
    // 多个引用同时修改
    let counters = vec![Counter::new(); 3];
    
    for counter in &counters {
        counter.increment();
    }
    
    for (i, counter) in counters.iter().enumerate() {
        println!("计数器 {} 的值: {}", i, counter.get());
    }
    
    println!();
}

fn cyclic_reference_example() {
    println!("=== 循环引用示例 ===");
    
    struct Node {
        value: i32,
        parent: RefCell<Option<Rc<Node>>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    
    // 创建节点
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(None),
        children: RefCell::new(vec![]),
    });
    
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(None),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    // 建立父子关系
    *leaf.parent.borrow_mut() = Some(Rc::clone(&branch));
    
    println!("叶子节点的父节点值: {:?}", leaf.parent.borrow().as_ref().map(|node| node.value));
    println!("分支节点的子节点值: {:?}", branch.children.borrow().iter().map(|node| node.value).collect::<Vec<_>>());
    
    // 检查引用计数
    println!("叶子节点的引用计数: {}", Rc::strong_count(&leaf));
    println!("分支节点的引用计数: {}", Rc::strong_count(&branch));
    
    println!();
}

fn advanced_usage() {
    println!("=== 高级用法 ===");
    
    // 1. RefCell 的 map 方法
    let cell = RefCell::new(vec![1, 2, 3, 4, 5]);
    
    // 使用 Ref::map 转换借用类型
    let first = RefCell::borrow(&cell);
    let first_elem = std::cell::Ref::map(first, |vec| &vec[0]);
    println!("第一个元素: {}", first_elem);
    
    // 2. 使用 RefMut::map 进行转换
    let mut mut_ref = cell.borrow_mut();
    let mut first_mut = std::cell::RefMut::map(mut_ref, |vec| &mut vec[0]);
    *first_mut += 10;
    drop(first_mut);
    
    println!("修改第一个元素后: {:?}", cell.borrow());
    
    // 3. 嵌套 RefCell
    let nested = RefCell::new(RefCell::new(42));
    *nested.borrow().borrow_mut() += 1;
    println!("嵌套 RefCell 值: {}", nested.borrow().borrow());
    
    // 4. 使用 RefCell 实现观察者模式
    observer_pattern_example();
    
    println!();
}

fn observer_pattern_example() {
    println!("=== 观察者模式示例 ===");
    
    trait Observer {
        fn update(&self, value: i32);
    }
    
    struct Subject {
        value: RefCell<i32>,
        observers: RefCell<Vec<Box<dyn Observer>>>,
    }
    
    impl Subject {
        fn new() -> Self {
            Subject {
                value: RefCell::new(0),
                observers: RefCell::new(vec![]),
            }
        }
        
        fn add_observer(&self, observer: Box<dyn Observer>) {
            self.observers.borrow_mut().push(observer);
        }
        
        fn set_value(&self, value: i32) {
            *self.value.borrow_mut() = value;
            self.notify_observers();
        }
        
        fn notify_observers(&self) {
            let current_value = *self.value.borrow();
            for observer in self.observers.borrow().iter() {
                observer.update(current_value);
            }
        }
    }
    
    struct ConcreteObserver {
        name: String,
    }
    
    impl Observer for ConcreteObserver {
        fn update(&self, value: i32) {
            println!("{} 收到更新: 新值为 {}", self.name, value);
        }
    }
    
    let subject = Subject::new();
    
    subject.add_observer(Box::new(ConcreteObserver { name: "观察者1".to_string() }));
    subject.add_observer(Box::new(ConcreteObserver { name: "观察者2".to_string() }));
    
    subject.set_value(42);
    subject.set_value(100);
}
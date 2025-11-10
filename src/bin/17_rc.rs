// 17_rc.rs

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("=== Rc<T> 引用计数智能指针学习示例 ===\n");
    
    // 1. 基础创建和使用
    basic_usage();
    
    // 2. 共享所有权
    shared_ownership();
    
    // 3. 引用计数操作
    reference_counting();
    
    // 4. Rc和内部可变性
    rc_with_interior_mutability();
    
    // 5. 高级用法和模式
    advanced_patterns();
}

// 1. 基础创建和使用
fn basic_usage() {
    println!("1. Rc基础创建和使用:");
    
    // 创建Rc
    let rc1 = Rc::new(42);
    println!("   Rc::new(42): {:?}", rc1);
    println!("   Rc值: {}", *rc1);
    
    // 克隆Rc（增加引用计数）
    let rc2 = Rc::clone(&rc1);
    println!("   克隆后Rc值: {}", *rc2);
    println!("   rc1和rc2是否指向同一数据: {}", Rc::ptr_eq(&rc1, &rc2));
    
    // 使用Rc::from从Box创建
    let boxed = Box::new("Hello Rc");
    let rc_from_box = Rc::from(boxed);
    println!("   从Box创建: {}", rc_from_box);
    
    // 获取引用计数
    let rc3 = Rc::clone(&rc1);
    println!("   引用计数: {}", Rc::strong_count(&rc1));
    
    // 解引用访问
    let value = *rc1;
    println!("   解引用值: {}", value);
    
    println!();
}

// 2. 共享所有权
fn shared_ownership() {
    println!("2. 共享所有权:");
    
    // 多个所有者共享数据
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    println!("   初始引用计数: {}", Rc::strong_count(&data));
    
    {
        let shared1 = Rc::clone(&data);
        println!("   克隆后引用计数: {}", Rc::strong_count(&data));
        
        {
            let shared2 = Rc::clone(&data);
            println!("   再次克隆引用计数: {}", Rc::strong_count(&data));
            
            // 所有克隆都指向同一数据
            println!("   shared1[0]: {}", shared1[0]);
            println!("   shared2长度: {}", shared2.len());
        }
        
        println!("   shared2离开作用域后引用计数: {}", Rc::strong_count(&data));
    }
    
    println!("   shared1离开作用域后引用计数: {}", Rc::strong_count(&data));
    
    // 函数间共享数据
    fn process_data(data: Rc<Vec<i32>>) -> i32 {
        data.iter().sum()
    }
    
    let sum = process_data(Rc::clone(&data));
    println!("   数据处理结果: {}", sum);
    
    // 结构体中的Rc
    #[derive(Debug)]
    struct SharedContainer {
        data: Rc<String>,
        id: u32,
    }
    
    let shared_string = Rc::new("Shared String".to_string());
    let container1 = SharedContainer {
        data: Rc::clone(&shared_string),
        id: 1,
    };
    let container2 = SharedContainer {
        data: Rc::clone(&shared_string),
        id: 2,
    };
    
    println!("   容器1: {:?}", container1);
    println!("   容器2: {:?}", container2);
    println!("   共享字符串引用计数: {}", Rc::strong_count(&shared_string));
    
    println!();
}

// 3. 引用计数操作
fn reference_counting() {
    println!("3. 引用计数操作:");
    
    // 创建Rc并观察引用计数变化
    let data = Rc::new(100);
    println!("   初始引用计数: {}", Rc::strong_count(&data));
    
    // 克隆增加引用计数
    let clone1 = Rc::clone(&data);
    println!("   克隆1次后: {}", Rc::strong_count(&data));
    
    let clone2 = Rc::clone(&data);
    println!("   克隆2次后: {}", Rc::strong_count(&data));
    
    // 使用try_unwrap获取唯一所有权
    let data2 = Rc::new(200);
    let clone3 = Rc::clone(&data2);
    
    // 此时有两个引用，try_unwrap会失败
    match Rc::try_unwrap(data2) {
        Ok(value) => println!("   try_unwrap成功: {}", value),
        Err(rc) => println!("   try_unwrap失败，还有{}个引用", Rc::strong_count(&rc)),
    }
    
    // 当只有一个引用时，try_unwrap会成功
    let single_rc = Rc::new(300);
    match Rc::try_unwrap(single_rc) {
        Ok(value) => println!("   单引用try_unwrap成功: {}", value),
        Err(_) => println!("   单引用try_unwrap失败"),
    }
    
    // 获取弱引用
    let weak_ref = Rc::downgrade(&clone1);
    println!("   强引用计数: {}", Rc::strong_count(&clone1));
    println!("   弱引用计数: {}", Rc::weak_count(&clone1));
    
    // 从弱引用升级
    match weak_ref.upgrade() {
        Some(rc) => println!("   弱引用升级成功: {}", rc),
        None => println!("   弱引用升级失败"),
    }
    
    println!();
}

// 4. Rc和内部可变性
fn rc_with_interior_mutability() {
    println!("4. Rc和内部可变性:");
    
    // Rc<RefCell<T>>模式
    let shared_data = Rc::new(RefCell::new(50));
    
    // 多个所有者可以修改数据
    let owner1 = Rc::clone(&shared_data);
    let owner2 = Rc::clone(&shared_data);
    
    println!("   初始值: {}", *owner1.borrow());
    
    // 通过owner1修改
    *owner1.borrow_mut() += 10;
    println!("   owner1修改后: {}", *owner2.borrow());
    
    // 通过owner2修改
    *owner2.borrow_mut() *= 2;
    println!("   owner2修改后: {}", *owner1.borrow());
    
    // 共享可变集合
    let shared_vec = Rc::new(RefCell::new(vec![1, 2, 3]));
    let vec_owner1 = Rc::clone(&shared_vec);
    let vec_owner2 = Rc::clone(&shared_vec);
    
    vec_owner1.borrow_mut().push(4);
    vec_owner2.borrow_mut().push(5);
    
    println!("   共享向量: {:?}", shared_vec.borrow());
    
    // 结构体中的Rc<RefCell<>>
    #[derive(Debug)]
    struct Counter {
        count: Rc<RefCell<i32>>,
    }
    
    impl Counter {
        fn new() -> Self {
            Counter {
                count: Rc::new(RefCell::new(0)),
            }
        }
        
        fn increment(&self) {
            *self.count.borrow_mut() += 1;
        }
        
        fn get(&self) -> i32 {
            *self.count.borrow()
        }
        
        fn share(&self) -> Counter {
            Counter {
                count: Rc::clone(&self.count),
            }
        }
    }
    
    let counter1 = Counter::new();
    let counter2 = counter1.share();
    
    counter1.increment();
    counter2.increment();
    counter1.increment();
    
    println!("   计数器1: {}", counter1.get());
    println!("   计数器2: {}", counter2.get());
    
    println!();
}

// 5. 高级用法和模式
fn advanced_patterns() {
    println!("5. 高级用法和模式:");
    
    // Rc<Self>模式
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: Option<Rc<Node>>,
        children: Vec<Rc<Node>>,
    }
    
    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                parent: None,
                children: Vec::new(),
            })
        }
        
        fn add_child(self: &Rc<Node>, child: Rc<Node>) {
            // 这里需要unsafe来修改Rc内部，实际应用中需要更谨慎的处理
            // 为了演示，我们使用RefCell
        }
    }
    
    // 使用Weak避免循环引用
    use std::rc::Weak;
    
    #[derive(Debug)]
    struct TreeNode {
        value: String,
        parent: RefCell<Weak<TreeNode>>,
        children: RefCell<Vec<Rc<TreeNode>>>,
    }
    
    impl TreeNode {
        fn new(value: String) -> Rc<Self> {
            Rc::new(TreeNode {
                value,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            })
        }
        
        fn add_child(parent: &Rc<TreeNode>, child: &Rc<TreeNode>) {
            *child.parent.borrow_mut() = Rc::downgrade(parent);
            parent.children.borrow_mut().push(Rc::clone(child));
        }
        
        fn get_parent(self: &Rc<TreeNode>) -> Option<Rc<TreeNode>> {
            self.parent.borrow().upgrade()
        }
    }
    
    let root = TreeNode::new("Root".to_string());
    let child1 = TreeNode::new("Child1".to_string());
    let child2 = TreeNode::new("Child2".to_string());
    
    TreeNode::add_child(&root, &child1);
    TreeNode::add_child(&root, &child2);
    
    println!("   树结构创建成功");
    println!("   Root子节点数: {}", root.children.borrow().len());
    println!("   Child1的父节点: {:?}", child1.get_parent().map(|p| &p.value));
    
    // 引用计数检查
    println!("   Root强引用: {}", Rc::strong_count(&root));
    println!("   Root弱引用: {}", Rc::weak_count(&root));
    println!("   Child1强引用: {}", Rc::strong_count(&child1));
    println!("   Child1弱引用: {}", Rc::weak_count(&child1));
    
    // Rc的迭代器
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    let sum: i32 = data.iter().sum();
    println!("   Rc<Vec>求和: {}", sum);
    
    // Rc的解构
    let single_rc = Rc::new("Unique".to_string());
    let string = Rc::try_unwrap(single_rc).expect("应该有唯一所有权");
    println!("   解构Rc获得所有权: {}", string);
    
    println!();
}
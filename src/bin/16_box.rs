// 16_box.rs

use std::ops::Deref;
use std::fmt;

fn main() {
    println!("=== Box<T> 智能指针学习示例 ===\n");
    
    // 1. 基础创建和使用
    basic_usage();
    
    // 2. 堆上分配
    heap_allocation();
    
    // 3. Deref解引用
    deref_coercion();
    
    // 4. Box和递归类型
    recursive_types();
    
    // 5. 高级用法
    advanced_usage();
}

// 1. 基础创建和使用
fn basic_usage() {
    println!("1. Box基础创建和使用:");
    
    // 创建Box
    let boxed_i32 = Box::new(42);
    let boxed_string = Box::new(String::from("Hello, Box!"));
    
    println!("   Box::new(42): {:?}", boxed_i32);
    println!("   Box::new(String): {:?}", boxed_string);
    
    // 访问Box中的值
    println!("   解引用值: {}", *boxed_i32);
    println!("   解引用字符串: {}", *boxed_string);
    
    // Box的所有权
    let boxed_vec = Box::new(vec![1, 2, 3, 4, 5]);
    println!("   Box<Vec>长度: {}", boxed_vec.len());
    
    // Box可以被移动
    let moved_box = boxed_vec;
    println!("   移动后的Box长度: {}", moved_box.len());
    
    // 创建空Box
    let empty_box: Box<i32> = Box::default();
    println!("   默认Box: {}", empty_box);
    
    println!();
}

// 2. 堆上分配
fn heap_allocation() {
    println!("2. 堆上分配:");
    
    // 大数组的堆分配
    let large_array = Box::new([0u8; 1000]);
    println!("   大数组Box大小: {} 字节", std::mem::size_of_val(&*large_array));
    println!("   Box本身大小: {} 字节", std::mem::size_of_val(&large_array));
    
    // 复杂数据结构的堆分配
    #[derive(Debug)]
    struct LargeStruct {
        data: [u64; 100],
        name: String,
    }
    
    let large_struct = Box::new(LargeStruct {
        data: [0; 100],
        name: "Large Structure".to_string(),
    });
    
    println!("   大结构体Box创建成功");
    println!("   结构体大小: {} 字节", std::mem::size_of_val(&*large_struct));
    
    // Box的内存布局
    let simple_box = Box::new(123);
    println!("   简单Box指向地址: {:p}", &*simple_box);
    println!("   Box本身地址: {:p}", &simple_box);
    
    // 从Box获取原始指针
    let raw_ptr = Box::into_raw(simple_box);
    println!("   原始指针: {:p}", raw_ptr);
    
    // 从原始指针恢复Box
    unsafe {
        let recovered_box = Box::from_raw(raw_ptr);
        println!("   恢复的值: {}", recovered_box);
    }
    
    println!();
}

// 3. Deref解引用
fn deref_coercion() {
    println!("3. Deref解引用和强制转换:");
    
    let boxed_value = Box::new(42);
    
    // 显式解引用
    println!("   显式解引用: {}", *boxed_value);
    
    // 隐式解引用（方法调用）
    let boxed_string = Box::new(String::from("Hello"));
    println!("   隐式解引用长度: {}", boxed_string.len());
    
    // 函数参数中的解引用强制转换
    fn print_value(x: &i32) {
        println!("   函数参数解引用: {}", x);
    }
    
    print_value(&boxed_value); // &Box<i32> 自动转换为 &i32
    
    // 自定义类型的Deref
    struct MyBox<T>(T);
    
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    
    impl<T> Deref for MyBox<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    let my_box = MyBox::new(100);
    println!("   自定义MyBox解引用: {}", *my_box);
    
    // Box的DerefMut
    let mut boxed_mut = Box::new(50);
    *boxed_mut += 10;
    println!("   可变Box修改后: {}", boxed_mut);
    
    println!();
}

// 4. Box和递归类型
fn recursive_types() {
    println!("4. Box和递归类型:");
    
    // 链表节点 - 递归类型需要Box
    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    // 创建链表
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("   链表: {:?}", list);
    
    // 二叉树
    #[derive(Debug)]
    struct TreeNode<T> {
        value: T,
        left: Option<Box<TreeNode<T>>>,
        right: Option<Box<TreeNode<T>>>,
    }
    
    impl<T> TreeNode<T> {
        fn new(value: T) -> Self {
            TreeNode {
                value,
                left: None,
                right: None,
            }
        }
        
        fn insert_left(&mut self, value: T) {
            self.left = Some(Box::new(TreeNode::new(value)));
        }
        
        fn insert_right(&mut self, value: T) {
            self.right = Some(Box::new(TreeNode::new(value)));
        }
    }
    
    let mut root = TreeNode::new(1);
    root.insert_left(2);
    root.insert_right(3);
    
    println!("   二叉树创建成功");
    
    // 递归计算链表长度
    fn list_length<T>(list: &List<T>) -> usize {
        match list {
            Cons(_, tail) => 1 + list_length(tail),
            Nil => 0,
        }
    }
    
    println!("   链表长度: {}", list_length(&list));
    
    // 递归打印树
    fn print_tree<T: fmt::Display>(node: &TreeNode<T>, depth: usize) {
        let indent = " ".repeat(depth * 2);
        println!("{}{}", indent, node.value);
        
        if let Some(left) = &node.left {
            print_tree(left, depth + 1);
        }
        if let Some(right) = &node.right {
            print_tree(right, depth + 1);
        }
    }
    
    println!("   二叉树结构:");
    print_tree(&root, 0);
    
    println!();
}

// 5. 高级用法
fn advanced_usage() {
    println!("5. Box高级用法:");
    
    // Box trait对象
    trait Animal {
        fn speak(&self);
    }
    
    struct Dog;
    struct Cat;
    
    impl Animal for Dog {
        fn speak(&self) {
            println!("   狗: 汪汪!");
        }
    }
    
    impl Animal for Cat {
        fn speak(&self) {
            println!("   猫: 喵喵!");
        }
    }
    
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
        Box::new(Dog),
    ];
    
    println!("   Trait对象示例:");
    for animal in &animals {
        animal.speak();
    }
    
    // Box的内存管理
    let mut boxed_vec = Box::new(vec![1, 2, 3, 4, 5]);
    println!("   原始向量: {:?}", boxed_vec);
    
    // 使用Box::leak获取'static引用
    let leaked: &'static mut Vec<i32> = Box::leak(boxed_vec);
    leaked.push(6);
    println!("   泄露后的向量: {:?}", leaked);
    
    // 注意：此时原始boxed_vec不能再使用，因为它已经被泄露
    
    // Box的克隆（需要T实现Clone）
    let original = Box::new(vec![1, 2, 3]);
    let cloned = original.clone();
    println!("   原始Box: {:?}", original);
    println!("   克隆Box: {:?}", cloned);
    
    // Box的默认实现
    let default_box: Box<i32> = Default::default();
    println!("   默认Box: {}", default_box);
    
    // Box的From实现
    let from_value = Box::from(42);
    println!("   From实现: {}", from_value);
    
    println!();
}
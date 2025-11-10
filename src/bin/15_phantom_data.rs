// 15_phantom_data.rs

use std::marker::PhantomData;
use std::fmt;

fn main() {
    println!("=== PhantomData<T> 学习示例 ===\n");
    
    // 1. 基础概念和创建
    basic_concepts();
    
    // 2. 生命周期标记
    lifetime_marker();
    
    // 3. 类型标记
    type_marker();
    
    // 4. 不安全代码中的应用
    unsafe_usage();
    
    // 5. 高级模式
    advanced_patterns();
}

// 1. 基础概念和创建
fn basic_concepts() {
    println!("1. PhantomData基础概念:");
    
    // PhantomData是一个零开销的标记类型
    let _phantom_i32: PhantomData<i32> = PhantomData;
    let _phantom_string: PhantomData<String> = PhantomData;
    
    println!("   PhantomData<i32> 创建成功");
    println!("   PhantomData<String> 创建成功");
    
    // PhantomData不占用内存
    println!("   PhantomData<i32> 大小: {} 字节", std::mem::size_of::<PhantomData<i32>>());
    println!("   i32 大小: {} 字节", std::mem::size_of::<i32>());
    
    // 可以创建不同变体
    let _phantom_ref: PhantomData<&i32> = PhantomData;
    let _phantom_mut_ref: PhantomData<&mut String> = PhantomData;
    
    println!("   PhantomData<&i32> 创建成功");
    println!("   PhantomData<&mut String> 创建成功");
    
    println!();
}

// 2. 生命周期标记
fn lifetime_marker() {
    println!("2. 作为生命周期标记:");
    
    // 问题：结构体需要生命周期参数但不存储引用
    struct Container<'a> {
        value: i32,
        _marker: PhantomData<&'a i32>, // 告诉编译器这个结构体使用生命周期'a'
    }
    
    impl<'a> Container<'a> {
        fn new(value: i32) -> Self {
            Container {
                value,
                _marker: PhantomData,
            }
        }
        
        fn get_value(&self) -> i32 {
            self.value
        }
    }
    
    let container = Container::new(42);
    println!("   Container值: {}", container.get_value());
    
    // 更复杂的生命周期标记示例
    struct StrHolder<'a> {
        len: usize,
        _marker: PhantomData<&'a str>, // 标记这个结构体与str的生命周期相关
    }
    
    impl<'a> StrHolder<'a> {
        fn from_str(s: &'a str) -> Self {
            StrHolder {
                len: s.len(),
                _marker: PhantomData,
            }
        }
        
        fn get_len(&self) -> usize {
            self.len
        }
    }
    
    let text = "Hello, PhantomData!";
    let holder = StrHolder::from_str(text);
    println!("   StrHolder长度: {}", holder.get_len());
    
    println!();
}

// 3. 类型标记
fn type_marker() {
    println!("3. 作为类型标记:");
    
    // 泛型类型参数需要被使用的情况
    struct GenericWrapper<T> {
        value: i32, // 实际存储的是i32，但需要标记T
        _marker: PhantomData<T>,
    }
    
    impl<T> GenericWrapper<T> {
        fn new(value: i32) -> Self {
            GenericWrapper {
                value,
                _marker: PhantomData,
            }
        }
        
        fn process(&self) -> i32 {
            println!("   处理类型: {}", std::any::type_name::<T>());
            self.value * 2
        }
    }
    
    let wrapper_i32 = GenericWrapper::<i32>::new(10);
    let wrapper_string = GenericWrapper::<String>::new(20);
    
    println!("   i32 wrapper处理结果: {}", wrapper_i32.process());
    println!("   String wrapper处理结果: {}", wrapper_string.process());
    
    // 类型状态模式
    struct StateMachine<State> {
        data: i32,
        _state: PhantomData<State>,
    }
    
    struct Initial;
    struct Processing;
    struct Completed;
    
    impl StateMachine<Initial> {
        fn new() -> Self {
            StateMachine {
                data: 0,
                _state: PhantomData,
            }
        }
        
        fn start(self, value: i32) -> StateMachine<Processing> {
            StateMachine {
                data: value,
                _state: PhantomData,
            }
        }
    }
    
    impl StateMachine<Processing> {
        fn process(&mut self) {
            self.data *= 2;
        }
        
        fn finish(self) -> StateMachine<Completed> {
            StateMachine {
                data: self.data,
                _state: PhantomData,
            }
        }
    }
    
    impl StateMachine<Completed> {
        fn result(&self) -> i32 {
            self.data
        }
    }
    
    let machine = StateMachine::new();
    let mut processing = machine.start(21);
    processing.process();
    let completed = processing.finish();
    println!("   状态机结果: {}", completed.result());
    
    println!();
}

// 4. 不安全代码中的应用
fn unsafe_usage() {
    println!("4. 在不安全代码中的应用:");
    
    // 原始指针的生命周期管理
    struct RawPointerHolder<T> {
        ptr: *const T,
        _marker: PhantomData<T>, // 告诉编译器我们拥有T的所有权
    }
    
    impl<T> RawPointerHolder<T> {
        fn new(value: &T) -> Self {
            RawPointerHolder {
                ptr: value as *const T,
                _marker: PhantomData,
            }
        }
        
        unsafe fn get(&self) -> &T {
            &*self.ptr
            // 注意：这里需要确保指针仍然有效
        }
    }
    
    let value = 42;
    let holder = RawPointerHolder::new(&value);
    unsafe {
        println!("   原始指针值: {}", holder.get());
    }
    
    // 自定义集合类型
    struct MyVec<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
        _marker: PhantomData<T>, // 标记我们拥有T的所有权
    }
    
    impl<T> MyVec<T> {
        fn new() -> Self {
            MyVec {
                ptr: std::ptr::null_mut(),
                len: 0,
                capacity: 0,
                _marker: PhantomData,
            }
        }
        
        fn len(&self) -> usize {
            self.len
        }
    }
    
    let my_vec: MyVec<i32> = MyVec::new();
    println!("   自定义Vec长度: {}", my_vec.len());
    
    println!();
}

// 5. 高级模式
fn advanced_patterns() {
    println!("5. 高级模式:");
    
    // 逆变和协变标记
    struct Contravariant<T> {
        _marker: PhantomData<fn(T)>, // 逆变
    }
    
    struct Covariant<T> {
        _marker: PhantomData<fn() -> T>, // 协变
    }
    
    struct Invariant<T> {
        _marker: PhantomData<fn(T) -> T>, // 不变
    }
    
    // 复杂生命周期场景
    struct BufferView<'data, T> {
        start: *const T,
        len: usize,
        _marker: PhantomData<&'data [T]>,
    }
    
    impl<'data, T> BufferView<'data, T> {
        fn from_slice(slice: &'data [T]) -> Self {
            BufferView {
                start: slice.as_ptr(),
                len: slice.len(),
                _marker: PhantomData,
            }
        }
        
        fn len(&self) -> usize {
            self.len
        }
    }
    
    let data = vec![1, 2, 3, 4, 5];
    let view = BufferView::from_slice(&data);
    println!("   BufferView长度: {}", view.len());
    
    // 零大小类型优化
    struct ZeroSized<T> {
        _marker: PhantomData<T>,
    }
    
    impl<T> ZeroSized<T> {
        fn new() -> Self {
            ZeroSized {
                _marker: PhantomData,
            }
        }
        
        fn process(&self) {
            println!("   处理零大小类型: {}", std::any::type_name::<T>());
        }
    }
    
    let zero_i32 = ZeroSized::<i32>::new();
    let zero_vec = ZeroSized::<Vec<String>>::new();
    
    println!("   ZeroSized<i32>大小: {} 字节", std::mem::size_of::<ZeroSized<i32>>());
    zero_i32.process();
    zero_vec.process();
    
    println!();
}
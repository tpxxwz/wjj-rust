use std::cell::Cell;
use std::rc::Rc;

fn main() {
    println!("=== Cell 学习示例 ===\n");

    // 基础 Cell 使用
    basic_cell();
    
    // Cell 与 Rc 结合使用
    cell_with_rc();
    
    // Copy trait 的限制
    copy_trait_limitation();
    
    // 内部可变性模式
    interior_mutability_pattern();
    
    // 性能对比
    performance_comparison();
    
    // 高级用法
    advanced_usage();
}

fn basic_cell() {
    println!("=== 基础 Cell 使用 ===");
    
    // 创建 Cell
    let cell = Cell::new(42);
    
    // 获取值
    println!("初始值: {}", cell.get());
    
    // 设置新值
    cell.set(100);
    println!("设置后的值: {}", cell.get());
    
    // 替换值并返回旧值
    let old_value = cell.replace(200);
    println!("替换前值: {}, 替换后值: {}", old_value, cell.get());
    
    // 使用 take（仅适用于 Copy 类型）
    let value = cell.take();
    println!("take 后的值: {}, Cell 现在的值: {}", value, cell.get());
    
    // 使用 update 方法（Rust 1.63+）
    cell.set(50);
    let old = cell.replace_with(|&mut old| old * 2);
    println!("update 后的旧值: {}, 新值: {}", old, cell.get());
    
    println!();
}

fn cell_with_rc() {
    println!("=== Cell 与 Rc 结合使用 ===");
    
    // 创建共享的可变整数
    let shared_counter = Rc::new(Cell::new(0));
    
    // 创建多个引用
    let counter1 = Rc::clone(&shared_counter);
    let counter2 = Rc::clone(&shared_counter);
    
    // 通过不同引用修改值
    counter1.set(10);
    println!("counter1 设置后: {}", counter2.get()); // 所有引用看到相同的值
    
    counter2.set(counter2.get() + 5);
    println!("counter2 增加后: {}", counter1.get());
    
    // 创建共享的 Cell 数组
    let shared_array = Rc::new([
        Cell::new(1),
        Cell::new(2),
        Cell::new(3),
    ]);
    
    let array1 = Rc::clone(&shared_array);
    let array2 = Rc::clone(&shared_array);
    
    array1[0].set(10);
    println!("修改数组第一个元素: {}", array2[0].get());
    
    println!();
}

fn copy_trait_limitation() {
    println!("=== Copy trait 的限制 ===");
    
    // Cell 只能用于实现了 Copy trait 的类型
    let int_cell = Cell::new(42i32);
    let bool_cell = Cell::new(true);
    let char_cell = Cell::new('A');
    
    println!("整数 Cell: {}", int_cell.get());
    println!("布尔 Cell: {}", bool_cell.get());
    println!("字符 Cell: {}", char_cell.get());
    
    // 对于复杂类型，需要使用 RefCell 或 Mutex
    // let string_cell = Cell::new(String::from("hello")); // 编译错误
    
    // 但是可以使用 Cell 包装 Copy 类型的数组
    let array_cell = Cell::new([1, 2, 3, 4, 5]);
    println!("数组 Cell: {:?}", array_cell.get());
    
    // 或者使用 Cell 的数组
    let cell_array = [
        Cell::new(1),
        Cell::new(2),
        Cell::new(3),
    ];
    
    cell_array[1].set(20);
    println!("Cell 数组: {:?}", cell_array.iter().map(|c| c.get()).collect::<Vec<_>>());
    
    println!();
}

fn interior_mutability_pattern() {
    println!("=== 内部可变性模式 ===");
    
    // 使用 Cell 实现线程不安全的计数器
    struct Counter {
        count: Cell<u32>,
    }
    
    impl Counter {
        fn new() -> Self {
            Counter {
                count: Cell::new(0),
            }
        }
        
        fn increment(&self) {
            let current = self.count.get();
            self.count.set(current + 1);
        }
        
        fn get(&self) -> u32 {
            self.count.get()
        }
        
        fn reset(&self) {
            self.count.set(0);
        }
    }
    
    let counter = Counter::new();
    
    counter.increment();
    counter.increment();
    println!("计数器值: {}", counter.get());
    
    counter.reset();
    println!("重置后计数器值: {}", counter.get());
    
    // 使用 Cell 实现状态机
    struct StateMachine {
        state: Cell<State>,
    }
    
    #[derive(Clone, Copy, Debug)]
    enum State {
        Idle,
        Running(u32),
        Stopped,
    }
    
    impl StateMachine {
        fn new() -> Self {
            StateMachine {
                state: Cell::new(State::Idle),
            }
        }
        
        fn start(&self, value: u32) {
            self.state.set(State::Running(value));
        }
        
        fn stop(&self) {
            self.state.set(State::Stopped);
        }
        
        fn reset(&self) {
            self.state.set(State::Idle);
        }
        
        fn get_state(&self) -> State {
            self.state.get()
        }
    }
    
    let sm = StateMachine::new();
    println!("初始状态: {:?}", sm.get_state());
    
    sm.start(42);
    println!("启动后状态: {:?}", sm.get_state());
    
    sm.stop();
    println!("停止后状态: {:?}", sm.get_state());
    
    println!();
}

fn performance_comparison() {
    println!("=== 性能对比 ===");
    
    // Cell vs RefCell 的性能差异
    
    // Cell 不需要运行时借用检查，性能更好
    let cell = Cell::new(0);
    
    // 快速更新
    for i in 0..5 {
        cell.set(cell.get() + i);
    }
    println!("Cell 最终值: {}", cell.get());
    
    // RefCell 需要运行时借用检查
    use std::cell::RefCell;
    let ref_cell = RefCell::new(0);
    
    for i in 0..5 {
        *ref_cell.borrow_mut() += i;
    }
    println!("RefCell 最终值: {}", *ref_cell.borrow());
    
    // Cell 适合简单的 Copy 类型
    // RefCell 适合复杂的非 Copy 类型
    
    println!("Cell 适用于: 简单的 Copy 类型，性能要求高");
    println!("RefCell 适用于: 复杂的非 Copy 类型，需要内部可变性");
    
    println!();
}

fn advanced_usage() {
    println!("=== 高级用法 ===");
    
    // 1. 使用 Cell 实现记忆化
    memoization_example();
    
    // 2. 使用 Cell 实现懒加载
    lazy_initialization();
    
    // 3. 使用 Cell 实现简单的状态管理
    state_management();
    
    // 4. Cell 的组合使用
    cell_composition();
    
    println!();
}

fn memoization_example() {
    println!("=== 记忆化示例 ===");
    
    struct Fibonacci {
        cache: Vec<Cell<Option<u64>>>,
    }
    
    impl Fibonacci {
        fn new(max_n: usize) -> Self {
            Fibonacci {
                cache: (0..=max_n).map(|_| Cell::new(None)).collect(),
            }
        }
        
        fn compute(&self, n: usize) -> u64 {
            if let Some(result) = self.cache[n].get() {
                println!("缓存命中: fib({}) = {}", n, result);
                return result;
            }
            
            let result = if n <= 1 {
                n as u64
            } else {
                self.compute(n - 1) + self.compute(n - 2)
            };
            
            self.cache[n].set(Some(result));
            println!("计算: fib({}) = {}", n, result);
            result
        }
    }
    
    let fib = Fibonacci::new(10);
    let result = fib.compute(6);
    println!("fib(6) = {}", result);
    
    // 再次计算，应该使用缓存
    let result2 = fib.compute(6);
    println!("fib(6) 再次计算 = {}", result2);
    
    println!();
}

fn lazy_initialization() {
    println!("=== 懒加载示例 ===");
    
    struct LazyValue {
        value: Cell<Option<i32>>,
        init_fn: Box<dyn Fn() -> i32>,
    }
    
    impl LazyValue {
        fn new<F: Fn() -> i32 + 'static>(init_fn: F) -> Self {
            LazyValue {
                value: Cell::new(None),
                init_fn: Box::new(init_fn),
            }
        }
        
        fn get(&self) -> i32 {
            if let Some(val) = self.value.get() {
                val
            } else {
                let new_val = (self.init_fn)();
                self.value.set(Some(new_val));
                new_val
            }
        }
    }
    
    let lazy = LazyValue::new(|| {
        println!("执行昂贵的初始化操作...");
        42
    });
    
    println!("第一次获取值:");
    let val1 = lazy.get();
    println!("获取到的值: {}", val1);
    
    println!("第二次获取值:");
    let val2 = lazy.get();
    println!("获取到的值: {}", val2);
    
    println!();
}

fn state_management() {
    println!("=== 状态管理示例 ===");
    
    struct StateManager {
        current_state: Cell<AppState>,
        previous_state: Cell<Option<AppState>>,
    }
    
    #[derive(Clone, Copy, Debug, PartialEq)]
    enum AppState {
        Loading,
        Ready(u32),
        Error,
    }
    
    impl StateManager {
        fn new() -> Self {
            StateManager {
                current_state: Cell::new(AppState::Loading),
                previous_state: Cell::new(None),
            }
        }
        
        fn transition_to(&self, new_state: AppState) {
            let current = self.current_state.get();
            self.previous_state.set(Some(current));
            self.current_state.set(new_state);
        }
        
        fn get_current(&self) -> AppState {
            self.current_state.get()
        }
        
        fn get_previous(&self) -> Option<AppState> {
            self.previous_state.get()
        }
        
        fn can_transition(&self, new_state: AppState) -> bool {
            let current = self.current_state.get();
            match (current, new_state) {
                (AppState::Loading, AppState::Ready(_)) => true,
                (AppState::Ready(_), AppState::Error) => true,
                (AppState::Error, AppState::Loading) => true,
                _ => false,
            }
        }
    }
    
    let sm = StateManager::new();
    println!("初始状态: {:?}", sm.get_current());
    
    if sm.can_transition(AppState::Ready(100)) {
        sm.transition_to(AppState::Ready(100));
        println!("转换到 Ready 状态: {:?}", sm.get_current());
    }
    
    println!("前一个状态: {:?}", sm.get_previous());
    
    println!();
}

fn cell_composition() {
    println!("=== Cell 组合使用 ===");
    
    // 多个 Cell 组合使用
    struct Point {
        x: Cell<i32>,
        y: Cell<i32>,
    }
    
    impl Point {
        fn new(x: i32, y: i32) -> Self {
            Point {
                x: Cell::new(x),
                y: Cell::new(y),
            }
        }
        
        fn move_to(&self, new_x: i32, new_y: i32) {
            self.x.set(new_x);
            self.y.set(new_y);
        }
        
        fn translate(&self, dx: i32, dy: i32) {
            self.x.set(self.x.get() + dx);
            self.y.set(self.y.get() + dy);
        }
        
        fn get_coordinates(&self) -> (i32, i32) {
            (self.x.get(), self.y.get())
        }
    }
    
    let point = Point::new(10, 20);
    println!("初始坐标: {:?}", point.get_coordinates());
    
    point.translate(5, -3);
    println!("平移后坐标: {:?}", point.get_coordinates());
    
    point.move_to(100, 200);
    println!("移动到新坐标: {:?}", point.get_coordinates());
    
    // 使用 Cell 实现简单的图形对象
    struct Rectangle {
        top_left: Point,
        width: Cell<u32>,
        height: Cell<u32>,
    }
    
    impl Rectangle {
        fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
            Rectangle {
                top_left: Point::new(x, y),
                width: Cell::new(width),
                height: Cell::new(height),
            }
        }
        
        fn resize(&self, new_width: u32, new_height: u32) {
            self.width.set(new_width);
            self.height.set(new_height);
        }
        
        fn move_to(&self, new_x: i32, new_y: i32) {
            self.top_left.move_to(new_x, new_y);
        }
        
        fn get_dimensions(&self) -> (u32, u32) {
            (self.width.get(), self.height.get())
        }
        
        fn get_position(&self) -> (i32, i32) {
            self.top_left.get_coordinates()
        }
    }
    
    let rect = Rectangle::new(10, 10, 50, 30);
    println!("矩形位置: {:?}, 尺寸: {:?}", rect.get_position(), rect.get_dimensions());
    
    rect.resize(100, 60);
    rect.move_to(50, 50);
    println!("调整后位置: {:?}, 尺寸: {:?}", rect.get_position(), rect.get_dimensions());
}
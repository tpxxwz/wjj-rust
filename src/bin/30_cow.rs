use std::borrow::Cow;

fn main() {
    println!("=== Cow (Clone-on-Write) 学习示例 ===\n");

    // 1. Cow 基础用法
    basic_usage();

    // 2. Cow 作为函数参数
    cow_as_function_parameter();

    // 3. Cow 避免不必要的分配
    avoid_unnecessary_allocations();

    // 4. Cow 在需要时进行分配
    allocate_when_needed();

    // 5. Cow 与 Vec
    cow_with_vec();
}

fn basic_usage() {
    println!("--- 1. Cow 基础用法 ---");

    // Cow 可以是借用的 (Borrowed)
    let borrowed_str: Cow<'static, str> = Cow::Borrowed("I am borrowed.");
    println!("Borrowed Cow: {}", borrowed_str);

    // Cow 也可以是拥有的 (Owned)
    let owned_str: Cow<'static, str> = Cow::Owned(String::from("I am owned."));
    println!("Owned Cow: {}", owned_str);

    // 使用 into() 转换
    let s1: String = String::from("hello");
    let cow_from_string: Cow<str> = s1.into();
    println!("Cow from String: {}", cow_from_string);

    let s2: &str = "world";
    let cow_from_str_slice: Cow<str> = s2.into();
    println!("Cow from &str: {}", cow_from_str_slice);

    println!();
}

fn cow_as_function_parameter() {
    println!("--- 2. Cow 作为函数参数 ---");

    // 传递一个借用的字符串
    process_data("immutable string".into());

    // 传递一个拥有的字符串
    process_data(String::from("mutable string").into());

    println!();
}

// 这个函数可以接受 &str 或 String
fn process_data(data: Cow<str>) {
    println!("Processing data: '{}'", data);
    match data {
        Cow::Borrowed(s) => println!("  - It's a borrowed string."),
        Cow::Owned(ref s) => println!("  - It's an owned string: {}", s),
    }
}

fn avoid_unnecessary_allocations() {
    println!("--- 3. Cow 避免不必要的分配 ---");

    let original_str = "No changes needed.";
    let processed_cow = process_or_clone(original_str.into());

    // 因为没有修改，所以 processed_cow 是借用的，没有发生内存分配
    if let Cow::Borrowed(s) = processed_cow {
        println!("Result is borrowed: '{}'", s);
        // 使用 ptr::eq 比较指针地址，确认是同一个字符串
        assert!(std::ptr::eq(original_str.as_ptr(), s.as_ptr()));
        println!("  - No allocation occurred.");
    } else {
        println!("Result is owned. This shouldn't happen in this case.");
    }

    println!();
}

fn allocate_when_needed() {
    println!("--- 4. Cow 在需要时进行分配 ---");

    let original_str = "Needs a change!";
    let mut processed_cow = process_or_clone(original_str.into());

    // 因为需要修改，所以 processed_cow 变成了拥有的，发生了内存分配
    if let Cow::Owned(ref s) = processed_cow {
        println!("Result is owned: '{}'", s);
        println!("  - Allocation occurred.");
    } else {
        println!("Result is borrowed. This shouldn't happen in this case.");
    }

    // to_mut() 会确保 Cow 是拥有的，如果不是，会克隆一份
    let mutable_ref = processed_cow.to_mut();
    mutable_ref.push_str(" And now it's modified.");
    println!("Modified Cow: '{}'", mutable_ref);

    println!();
}

// 这个函数模拟一个场景：如果字符串包含 "change"，就替换它，否则返回原样
fn process_or_clone(input: Cow<str>) -> Cow<str> {
    if input.contains("change") {
        // 需要修改，所以克隆成 String 并替换
        let owned_string = input.replace("change", "modification");
        Cow::Owned(owned_string)
    } else {
        // 不需要修改，直接返回借用的 Cow
        input
    }
}

fn cow_with_vec() {
    println!("--- 5. Cow 与 Vec ---");

    let numbers = vec![1, 2, 3, 4, 5];

    // 场景1: 不需要修改，返回借用的 slice
    let processed_vec1 = process_vec(Cow::Borrowed(&numbers));
    if let Cow::Borrowed(slice) = processed_vec1 {
        println!("Processed vec (no change): {:?}", slice);
        assert!(std::ptr::eq(numbers.as_ptr(), slice.as_ptr()));
        println!("  - No allocation occurred.");
    }

    // 场景2: 需要修改，返回拥有的 Vec
    let processed_vec2 = process_vec(Cow::Owned(vec![10, 20, 30]));
    if let Cow::Owned(ref vec) = processed_vec2 {
        println!("Processed vec (with change): {:?}", vec);
        println!("  - Allocation occurred.");
    }
    println!();
}

// 如果 slice 中有大于 10 的数，则返回一个过滤后的新 Vec，否则返回原 slice
fn process_vec(data: Cow<[i32]>) -> Cow<[i32]> {
    if data.iter().any(|&x| x > 10) {
        // 需要修改，创建一个新的 Vec
        let filtered: Vec<i32> = data.iter().filter(|&&x| x <= 10).cloned().collect();
        Cow::Owned(filtered)
    } else {
        // 不需要修改，返回借用的 slice
        data
    }
}
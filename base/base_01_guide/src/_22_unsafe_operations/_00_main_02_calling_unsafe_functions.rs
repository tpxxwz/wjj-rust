// 调用不安全函数
// 某些函数可以被声明为 unsafe，这意味着确保其正确性是程序员的责任，而不是编译器的责任。
// 一个例子是 std::slice::from_raw_parts，它根据指向第一个元素的指针和长度创建一个切片

use std::slice;

#[wjj_lib::gen_test]
fn main() {
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }
}

// 对于 slice::from_raw_parts，必须遵守的一个假设是：传入的指针指向有效内存，且指向的内存类型正确。
// 如果这些不变量未被遵守，那么程序的行为将是未定义的，无法预知会发生什么。


// Result
// Result 是 Option 类型的增强版，它描述可能的错误而非可能的缺失。
//
// 也就是说，Result<T, E> 可能有两种结果之一：
//
// Ok(T)：找到了一个 T 类型的元素
// Err(E)：发生了一个 E 类型的错误
// 按照惯例，预期的结果是 Ok，而意外的结果是 Err。
//
// 与 Option 类似，Result 也有许多关联方法。
// 例如，unwrap() 要么返回元素 T，要么触发 panic。
// 对于情况处理，Result 和 Option 之间有许多重叠的组合子。
//
// 在使用 Rust 时，你可能会遇到返回 Result 类型的方法，比如 parse() 方法。
// 将字符串解析为其他类型并非总是可行，因此 parse() 返回一个 Result 来表示可能的失败。
//
// 让我们看看成功和失败地 parse() 一个字符串会发生什么：

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // 让我们尝试使用 `unwrap()` 来获取数字。这样做会有问题吗？
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}
// 在解析失败的情况下，parse() 会返回一个错误，导致 unwrap() 触发 panic。此外，panic 会终止程序并输出一条不友好的错误信息。
//
// 为了提高错误信息的质量，我们应该更明确地指定返回类型，并考虑显式地处理错误。

crate::gen_test!(main);
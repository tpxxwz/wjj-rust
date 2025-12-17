// 函数
// 排除省略（elision）的情况，带有生命周期的函数签名有以下几个约束：
//
// 任何引用必须有一个标注的生命周期。
// 任何被返回的引用必须具有与输入相同的生命周期或者是 static。
// 此外，请注意，如果返回引用而没有输入会导致返回指向无效数据的引用，这是被禁止的。
// 以下示例展示了一些带有生命周期的有效函数形式：

// 一个具有生命周期 `'a` 的输入引用，它必须至少与函数存活一样长。
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`：x 是 {}", x);
}

// 可变引用也可以有生命周期。
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// 多个具有不同生命周期的元素。在这种情况下，
// 两者都有相同的生命周期 `'a` 是可以的，但
// 在更复杂的情况下，可能需要不同的生命周期。
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`：x 是 {}，y 是 {}", x, y);
}

// 返回已传入的引用是可以接受的。
// 但是，必须返回正确的生命周期。
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// 上面的代码是无效的：`'a` 必须比函数存活更长。
// 这里的 `&String::from("foo")` 会创建一个 `String`，然后创建一个引用。
// 接着，在退出作用域时数据被丢弃，导致返回一个指向无效数据的引用。

#[test]
fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

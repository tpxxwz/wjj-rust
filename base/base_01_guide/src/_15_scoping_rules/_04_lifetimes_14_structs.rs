// 结构体
// 结构体中生命周期的注解也与函数类似：

// `Borrowed` 类型包含一个 `i32` 的引用。
// 这个 `i32` 的引用必须比 `Borrowed` 存活更久。
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// 同样，这里的两个引用都必须比这个结构体存活更久。
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// 一个枚举，可以是 `i32`，或者是 `i32` 的引用。
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

#[wjj_lib::gen_test]
fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x 在 {:?} 中被借用", single);
    println!("x 和 y 在 {:?} 中被借用", double);
    println!("x 在 {:?} 中被借用", reference);
    println!("y 在 {:?} 中**没有**被借用", number);
}


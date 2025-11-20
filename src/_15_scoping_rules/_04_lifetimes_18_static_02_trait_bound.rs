// Trait 约束
// 作为 trait 约束时，它表示该类型不包含任何非静态引用。例如，接收者可以随意持有该类型，直到主动丢弃之前，它都不会变为无效。
//
// 理解这一点很重要：任何拥有所有权的数据总是满足 'static 生命周期约束，但对该数据的引用通常不满足：

use std::fmt::Debug;

fn print_it( input: impl Debug + 'static ) {
    println!( "传入的 'static 值是：{:?}", input );
}

fn main() {
    // i 拥有所有权且不包含任何引用，因此它是 'static 的：
    let i = 5;
    print_it(i);

    // 糟糕，&i 的生命周期仅由 main() 的作用域定义，
    // 所以它不是 'static 的：
    // print_it(&i);
}

crate::gen_test!(main);
// 多重约束
// 可以使用 + 为单个类型指定多个约束。按照惯例，不同的类型用 , 分隔。

use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug：`{:?}`", t);
    println!("Display：`{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t：`{:?}`", t);
    println!("u：`{:?}`", u);
}

#[wjj_lib::gen_test]
fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // TODO：尝试取消此行注释。

    compare_types(&array, &vec);
}


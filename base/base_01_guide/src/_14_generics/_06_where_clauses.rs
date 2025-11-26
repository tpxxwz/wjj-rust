// Where 分句
// 约束也可以使用 where 子句来表达，它位于开括号 { 之前，而不是在类型首次提及时。
// 此外，where 子句可以将约束应用于任意类型，而不仅限于类型参数。
//
// where 子句在以下情况下特别有用：
//
// 当单独指定泛型类型和约束更清晰时：
// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
//
// // 使用 `where` 子句表达约束
// impl <A, D> MyTrait<A, D> for YourType where
//     A: TraitB + TraitC,
//     D: TraitE + TraitF {}

// 当使用 where 子句比使用普通语法更具表现力时。这个例子中的 impl 如果不使用 where 子句就无法直接表达：

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// 这里需要一个 `where` 子句：否则就必须将其表达为 `T: Debug` 或
// 使用另一种间接方法，
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // 我们需要 `Option<T>: Debug` 作为我们的约束，因为这是
    // 正在被打印的内容。否则就会使用错误的约束。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

#[wjj_lib::gen_test]
fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}


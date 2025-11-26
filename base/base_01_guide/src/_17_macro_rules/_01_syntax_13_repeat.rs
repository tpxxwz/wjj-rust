// 重复
// 宏可以在参数列表中使用 + 来表示一个参数可能重复至少一次，或使用 * 来表示一个参数可能重复零次或多次。
//
// 在下面的例子中，用 $(...),+ 包围匹配器将匹配一个或多个由逗号分隔的表达式。
// 另外请注意，最后一个情况的分号是可选的。

// `find_min!` 将计算任意数量参数中的最小值。
macro_rules! find_min {
    // 基本情况：
    ($x:expr) => ($x);
    // `$x` 后面至少跟着一个 `$y,`
    ($x:expr, $($y:expr),+) => (
        // 对剩余的 `$y` 递归调用 `find_min!`
        std::cmp::min($x, find_min!($($y),+))
    )
}

#[wjj_lib::gen_test]
fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}


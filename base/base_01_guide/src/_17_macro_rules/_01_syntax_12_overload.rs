// 重载
// 宏可以被重载以接受不同的参数组合。在这方面，macro_rules! 的工作方式类似于 match 块：

// `test!` 将以不同的方式比较 `$left` 和 `$right`
// 具体取决于你如何调用它：
macro_rules! test {
    // 参数不需要用逗号分隔。
    // 可以使用任何模板！
    ($left:expr; and $right:expr) => {
        println!("{:?} 和 {:?} 是 {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ 每个分支必须以分号结束。
    ($left:expr; or $right:expr) => {
        println!("{:?} 或 {:?} 是 {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}

#[wjj_lib::gen_test]
fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}


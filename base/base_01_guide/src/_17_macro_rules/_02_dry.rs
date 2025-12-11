// DRY(不要重复自己) (Don't Repeat Yourself)
// 宏通过提取函数和/或测试套件的共同部分，使我们能够编写符合 DRY（Don't Repeat Yourself）原则的代码。
// 下面是一个在 Vec<T> 上实现并测试 +=、*= 和 -= 运算符的示例：

use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // `tt`（token tree，标记树）指示符用于
    // 运算符和标记。
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}：维度不匹配：{:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}

// 实现 `add_assign`、`mul_assign` 和 `sub_assign` 函数。
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        };
    }

    // 测试 `add_assign`、`mul_assign` 和 `sub_assign`。
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}

// $ rustc --test _02_dry.rs && ./_02_dry
// running 3 tests
// test test::mul_assign ... ok
// test test::add_assign ... ok
// test test::sub_assign ... ok
//
// test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured

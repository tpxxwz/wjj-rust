// 'static 引用只需在程序生命周期的剩余部分有效，因此可以在程序执行过程中创建。
// 为了演示这一点，下面的例子使用 Box::leak 动态创建 'static 引用。
// 在这种情况下，它显然不会存在于整个程序生命周期，而只是从泄漏点开始存在。

extern crate rand;
use rand::{Fill, Rng};

fn random_vec() -> &'static [usize; 100] {
    let mut rng = rand::rng();
    let mut boxed = Box::new([0; 100]);
    for x in boxed.iter_mut() {
        *x = rng.random_range(0..usize::MAX);
    }
    Box::leak(boxed)
}

fn main() {
    let first: &'static [usize; 100] = random_vec();
    let second: &'static [usize; 100] = random_vec();
    assert_ne!(first, second)
}

crate::gen_test!(main);

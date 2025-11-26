// 特质
// trait 方法中的生命周期注解基本上与函数类似。注意，impl 也可能需要生命周期注解。

// 一个带有生命周期注解的结构体。
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// 为 impl 添加生命周期注解。
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

#[wjj_lib::gen_test]
fn main() {
    let b: Borrowed = Default::default();
    println!("b 是 {:?}", b);
}


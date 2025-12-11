// 方法
// 方法的生命周期注解与函数类似：

struct Owner(i32);

impl Owner {
    // 像独立函数一样注解生命周期。
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("`print`：{}", self.0);
    }
}

#[test]
fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}

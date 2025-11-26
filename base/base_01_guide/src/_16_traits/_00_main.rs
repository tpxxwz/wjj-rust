// 特质
// trait 是为未知类型 Self 定义的一组方法集合。这些方法可以访问同一 trait 中声明的其他方法。
//
// trait 可以为任何数据类型实现。在下面的例子中，我们定义了 Animal，一组方法的集合。
// 然后为 Sheep 数据类型实现 Animal trait，这样就可以对 Sheep 使用 Animal 中的方法。

struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // 关联函数签名；`Self` 指代实现者类型。
    fn new(name: &'static str) -> Self;

    // 方法签名；这些方法将返回一个字符串。
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // trait 可以提供默认的方法实现。
    fn talk(&self) {
        println!("{} 说 {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // 实现者的方法可以使用实现者的 trait 方法。
            println!("{} 已经剃过毛了...", self.name());
        } else {
            println!("{} 剃了个毛！", self.name);

            self.naked = true;
        }
    }
}

// 为 `Sheep` 实现 `Animal` trait
impl Animal for Sheep {
    // `Self` 是实现者类型，即 `Sheep`
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "咩～？"
        } else {
            "咩～！"
        }
    }

    // 可以重写默认的 trait 方法
    fn talk(&self) {
        // 例如，我们可以添加一些安静的思考
        println!("{} 短暂停顿…… {}", self.name, self.noise());
    }
}

#[wjj_lib::gen_test]
fn main() {
    // 在这种情况下需要类型标注
    let mut dolly: Sheep = Animal::new("多莉");
    // TODO ^ 尝试移除类型标注

    dolly.talk();
    dolly.shear();
    dolly.talk();
}


// 使用 dyn 返回 trait
// Rust 编译器需要知道每个函数的返回类型所需的内存空间。这意味着所有函数都必须返回一个具体类型。
// 与其他语言不同，如果你有一个像 Animal 这样的 trait，你不能编写一个直接返回 Animal 的函数，
// 因为它的不同实现可能需要不同大小的内存。
//
// 然而，有一个简单的解决方法。我们可以让函数返回一个包含 Animal 的 Box，而不是直接返回 trait 对象。
// Box 本质上是一个指向堆内存的引用。由于引用的大小是静态已知的，且编译器可以保证它指向堆上分配的 Animal，
// 这样我们就能从函数中返回一个 trait 了！
//
// Rust 在堆上分配内存时力求明确。
// 因此，如果你的函数以这种方式返回一个指向堆上 trait 的指针，
// 你需要在返回类型中使用 dyn 关键字，例如 Box<dyn Animal>。

struct Sheep {}
struct Cow {}

trait Animal {
    // 实例方法签名
    fn noise(&self) -> &'static str;
}

// 为 `Sheep` 实现 `Animal` trait
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "咩～！"
    }
}

// 为 `Cow` 实现 `Animal` trait
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "哞~哞~哞~"
    }
}

// 返回某个实现了 Animal 的结构体，但在编译时我们并不知道具体是哪一个
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

#[test]
fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("你随机选择了一个动物，它说：{}", animal.noise());
}

// 派生
// 编译器可以通过 #[derive] 属性为某些 trait 提供基本实现。如果需要更复杂的行为，这些 trait 仍然可以手动实现。
//
// 以下是可派生的 trait 列表：
//
// 比较 trait：Eq、PartialEq、Ord、PartialOrd。
// Clone，通过复制 &T 创建 T。
// Copy，使类型具备"复制语义"而不是"移动语义"。
// Hash，从 &T 计算哈希值。
// Default，创建一个数据类型的空实例。
// Debug，使用 {:?} 格式化器来格式化一个值。

// `Centimeters`，一个可比较的元组结构体
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`，一个可打印的元组结构体
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`，一个没有额外属性的元组结构体
struct Seconds(i32);

#[wjj_lib::gen_test]
fn main() {
    let _one_second = Seconds(1);

    // 错误：`Seconds` 无法打印，因为它没有实现 `Debug` trait
    //println!("One second looks like: {:?}", _one_second);
    // TODO ^ 尝试取消这行的注释

    // 错误：`Seconds` 无法比较，因为它没有实现 `PartialEq` trait
    //let _this_is_true = (_one_second == _one_second);
    // TODO ^ 尝试取消这行的注释

    let foot = Inches(12);

    println!("一英尺等于 {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "更小"
        } else {
            "更大"
        };

    println!("一英尺比一米{}", cmp);
}


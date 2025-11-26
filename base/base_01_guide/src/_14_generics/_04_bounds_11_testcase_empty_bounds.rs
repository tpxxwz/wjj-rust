// 测试实例：空约束
// 约束的工作机制导致即使一个 trait 不包含任何功能，你仍然可以将其用作约束。
// std 库中的 Eq 和 Copy 就是这种 trait 的例子。

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 这些函数只对实现了这些 trait 的类型有效。
// 这些 trait 是否为空并不重要。
fn red<T: Red>(_: &T)   -> &'static str { "红色" }
fn blue<T: Blue>(_: &T) -> &'static str { "蓝色" }

#[wjj_lib::gen_test]
fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // 由于约束的存在，`red()` 不能用于蓝松鸟，反之亦然。
    println!("红雀是{}", red(&cardinal));
    println!("蓝松鸟是{}", blue(&blue_jay));
    //println!("火鸡是{}", red(&_turkey));
    // ^ TODO：尝试取消这行的注释。
}


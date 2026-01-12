// 测试实例：单位澄清
// 通过使用虚类型参数实现 Add trait，我们可以探索一种有用的单位转换方法。下面我们来看看 Add trait：

// 这个结构会强制要求：`Self + RHS = Output`
// 其中，如果在实现中未指定 RHS，它将默认为 Self。
// pub trait Add<RHS = Self> {
//     type Output;
//
//     fn add(self, rhs: RHS) -> Self::Output;
// }
//
// // `Output` 必须是 `T<U>`，以确保 `T<U> + T<U> = T<U>`。
// impl<U> Add for T<U> {
//     type Output = T<U>;
//     ...
// }

use std::marker::PhantomData;
use std::ops::Add;

/// 创建空枚举以定义单位类型。
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length` 是一个带有虚类型参数 `Unit` 的类型，
/// 它不是长度类型（即 `f64`）的泛型。
///
/// `f64` 已经实现了 `Clone` 和 `Copy` trait。
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// `Add` trait 定义了 `+` 运算符的行为。
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    // add() 返回一个包含和的新 `Length` 结构体。
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` 调用 `f64` 的 `Add` 实现。
        Length(self.0 + rhs.0, PhantomData)
    }
}

#[test]
fn main() {
    // 指定 `one_foot` 具有虚类型参数 `Inch`。
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    // `one_meter` 具有虚类型参数 `Mm`。
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // `+` 调用我们为 `Length<Unit>` 实现的 `add()` 方法。
    //
    // 由于 `Length` 实现了 `Copy`，`add()` 不会消耗
    // `one_foot` 和 `one_meter`，而是将它们复制到 `self` 和 `rhs` 中。
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // 加法运算正常工作。
    println!("一英尺 + 一英尺 = {:?} 英寸", two_feet.0);
    println!("一米 + 一米 = {:?} 毫米", two_meters.0);

    // 无意义的操作会按预期失败：
    // 编译时错误：类型不匹配。
    //let one_feter = one_foot + one_meter;
}

// 特质
// 当然，trait 也可以是泛型的。
// 这里我们定义了一个泛型 trait，它重新实现了 Drop trait，用于释放自身和一个输入参数。

// 不可复制的类型。
struct Empty;
struct Null;

// 一个泛型 trait，使用类型参数 `T`。
trait DoubleDrop<T> {
    // 在调用者类型上定义一个方法，该方法接受一个
    // 额外的类型为 `T` 的参数，但不对其进行任何操作。
    fn double_drop(self, _: T);
}

// 为任意泛型参数 `T` 和调用者 `U` 实现 `DoubleDrop<T>`。
impl<T, U> DoubleDrop<T> for U {
    // 此方法获取两个传入参数的所有权，
    // 并释放它们的内存。
    fn double_drop(self, _: T) {}
}

#[test]
fn main() {
    let empty = Empty;
    let null = Null;

    // 释放 `empty` 和 `null` 的内存。
    empty.double_drop(null);

    //empty;
    //null;
    // ^ TODO：尝试取消这些行的注释。
}

// 虚类型参数
// 虚类型参数是一种在运行时不会出现，但在编译时会进行静态检查的类型参数。
//
// 数据类型可以使用额外的泛型类型参数作为标记，或在编译时进行类型检查。这些额外的参数不占用存储空间，也没有运行时行为。
//
// 在下面的示例中，我们将 std::marker::PhantomData 与虚类型参数的概念结合，创建包含不同数据类型的元组。

use std::marker::PhantomData;

// 一个虚元组结构体，它在 `A` 上是泛型的，带有隐藏参数 `B`。
#[derive(PartialEq)] // 允许对此类型进行相等性测试。
struct PhantomTuple<A, B>(A, PhantomData<B>);

// 一个虚类型结构体，它在 `A` 上是泛型的，带有隐藏参数 `B`。
#[derive(PartialEq)] // 允许对此类型进行相等性测试。
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// 注意：为泛型类型 `A` 分配了存储空间，但没有为 `B` 分配。
//       因此，`B` 不能用于计算。

#[test]
fn main() {
    // 这里，`f32` 和 `f64` 是隐藏参数。
    // PhantomTuple 类型指定为 `<char, f32>`。
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // PhantomTuple 类型指定为 `<char, f64>`。
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // 类型指定为 `<char, f32>`。
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // 类型指定为 `<char, f64>`。
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 编译时错误！类型不匹配，无法比较：
    // println!("_tuple1 == _tuple2 的结果是：{}",
    //           _tuple1 == _tuple2);

    // 编译时错误！类型不匹配，无法比较：
    // println!("_struct1 == _struct2 的结果是：{}",
    //           _struct1 == _struct2);
}

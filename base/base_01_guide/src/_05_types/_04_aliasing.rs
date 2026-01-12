// 别名
// type 语句用于为现有类型创建新名称。
// 类型名必须使用 UpperCamelCase（大驼峰）命名，否则编译器会发出警告。
// 此规则的例外是原始类型，如 usize、f32 等。

// `NanoSecond`、`Inch` 和 `U64` 都是 `u64` 的新名称。
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

#[test]
fn main() {
    // `NanoSecond` = `Inch` = `U64` = `u64`。
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    // 注意，类型别名*不会*提供额外的类型安全性，因为别名*不是*新类型
    println!(
        "{} 纳秒 + {} 英寸 = {} 单位？",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
// 别名的主要用途是减少重复代码。例如，io::Result<T> 类型是 Result<T, io::Error> 类型的别名。

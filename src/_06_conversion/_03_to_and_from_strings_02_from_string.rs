// 解析字符串
// 将字符串转换为其他类型很有用，其中最常见的操作之一是将字符串转换为数字。
// 惯用的方法是使用 parse 函数，可以通过类型推断或使用"涡轮鱼"语法指定要解析的类型。以下示例展示了这两种方法。
//
// 只要为目标类型实现了 FromStr 特质，就可以将字符串转换为指定的类型。
// 标准库中为许多类型实现了这个特质。

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

// 要在自定义类型上获得这个功能，只需为该类型实现 FromStr 特质。
impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("总和：{:?}", sum);

    let radius = "    3 ";
    let circle: Circle = radius.parse().unwrap();
    println!("{:?}", circle);
}

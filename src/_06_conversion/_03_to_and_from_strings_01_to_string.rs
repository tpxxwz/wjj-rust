// 转换为字符串
// 要将任何类型转换为 String，只需为该类型实现 ToString 特质即可。
// 但更好的做法是实现 fmt::Display 特质，它不仅会自动提供 ToString，还允许打印该类型，就像在 print! 部分讨论的那样。

use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "半径为 {} 的圆", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}

crate::gen_test!(main);
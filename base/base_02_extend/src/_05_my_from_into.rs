// 定义两个 trait
trait MyFromStr {
    fn my_from_str(s: &str) -> Self;
}

trait MyIntoString {
    fn my_into_string(self) -> String;
}

trait MyIntoVec {
    fn my_into_vec(self) -> Vec<u8>;
}

// 为 String 类型实现一个自定义的 trait MyFromStr，并定义如何从 &str 转换成 String。
// 手动实现 MyFromStr
impl MyFromStr for String {
    fn my_from_str(s: &str) -> Self {
        s.to_string()
    }
}

// 为 Vec<u8> 类型实现一个自定义的 trait MyFromStr，并定义如何从 &str 转换成 Vec<u8>。
impl MyFromStr for Vec<u8> {
    fn my_from_str(s: &str) -> Self {
        s.as_bytes().to_vec()
    }
}

// 手动实现 MyIntoString（只能返回 String）
impl MyIntoString for &str {
    fn my_into_string(self) -> String {
        String::my_from_str(self)
    }
}

// 手动实现 MyIntoVec（只能返回 Vec<u8>）
impl MyIntoVec for &str {
    fn my_into_vec(self) -> Vec<u8> {
        Vec::<u8>::my_from_str(self)
    }
}

fn main() {
    let s: String = "hello".my_into_string();
    let v: Vec<u8> = "hello".my_into_vec();

    println!("String: {}", s);
    println!("Vec<u8>: {:?}", v);
}

#[test]
fn from_into_no_blanket_impl() {
    let s: String = "hello".my_into_string();
    let v: Vec<u8> = "hello".my_into_vec();

    println!("String: {}", s);
    println!("Vec<u8>: {:?}", v);
}
// 如果T为 &str U为Vec<u8>
// 也就是我有一个&str对象能变成Vec<u8>对象 自定义的MyFrom
// 通过下面的blanket impl
// 那就是 &str.my_into能变成Vec<u8>对象

trait MyFrom<T> {
    fn my_from(value: T) -> Self;
}

trait MyInto<T> {
    fn my_into(self) -> T;
}

// 然后写一个 blanket impl：
// 只要某个类型实现了 MyFrom<T>，就自动获得 MyInto<U>，并且可以像 .into() 一样根据上下文返回不同类型。

impl<T, U> MyInto<U> for T
where
    U: MyFrom<T>,
{
    fn my_into(self) -> U {
        U::my_from(self)
    }
}

// 实现 MyFrom
// 能够定义From<我有的类型> 我想变成的类型
// 自动给我有的类型生成一个my_into方法 方法返回我想变成的类型
// **** 并非我想变成的类型 变成我有的类型 ****
// **** 只是我有的类型 变成我想变的类型 ****
impl MyFrom<&str> for String {
    fn my_from(value: &str) -> Self {
        value.to_string()
    }
}

impl MyFrom<&str> for Vec<u8> {
    fn my_from(value: &str) -> Self {
        value.as_bytes().to_vec()
    }
}
#[test]
fn my_from_into() {
    let s: String = "hello".my_into();
    let v: Vec<u8> = "hello".my_into();
    println!("String: {}", s);
    println!("Vec<u8>: {:?}", v);
}

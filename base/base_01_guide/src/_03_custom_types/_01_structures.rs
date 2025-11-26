// 结构体
// 使用 struct 关键字可以创建三种类型的结构体（"structs"）：
//
// 元组结构体：本质上是具名元组。
// 经典的 C 语言风格结构体
// 单元结构体：没有字段，在泛型中很有用。

// 这个属性用于隐藏未使用代码的警告。
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单元结构体
struct Unit;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
struct Rectangle {
    // 可以通过指定左上角和右下角的位置
    // 来定义一个矩形。
    top_left: Point,
    bottom_right: Point,
}

#[wjj_lib::gen_test]
fn main() {
    // 使用字段初始化简写语法创建结构体
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // 打印结构体的调试信息
    println!("{:?}", peter);

    // 实例化一个 `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // 访问点的字段
    println!("点的坐标：({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的点，
    // 复用之前创建的点的字段
    let bottom_right = Point { x: 10.3, ..another_point };

    // `bottom_right.y` 与 `another_point.y` 相同，
    // 因为我们使用了 `another_point` 的该字段
    println!("第二个点：({}, {})", bottom_right.x, bottom_right.y);

    // 使用 `let` 绑定解构点
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // 结构体实例化也是一个表达式
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair 包含 {:?} 和 {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair 包含 {:?} 和 {:?}", integer, decimal);
}


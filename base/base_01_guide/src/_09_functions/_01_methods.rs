// 关联函数和方法
// 某些函数与特定类型相关联。
// 这些函数有两种形式：关联函数和方法。
// 关联函数是在类型上定义的函数，而方法是在类型的特定实例上调用的关联函数。

struct Point {
    x: f64,
    y: f64,
}

// 实现块，所有 `Point` 的关联函数和方法都在此处定义
impl Point {
    // 这是一个"关联函数"，因为这个函数与特定类型 Point 相关联。
    //
    // 关联函数不需要通过实例来调用。
    // 这些函数通常用作构造函数。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 另一个接受两个参数的关联函数：
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 这是一个方法
    // `&self` 是 `self: &Self` 的语法糖，其中 `Self` 是调用者对象的类型。
    // 在这个例子中 `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` 通过点运算符访问结构体字段
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` 是 `f64` 类型的方法，返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 这个方法要求调用对象是可变的
    // `&mut self` 是 `self: &mut Self` 的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` 拥有两个堆分配的整数资源
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 这个方法会"消耗"调用对象的资源
    // `self` 是 `self: Self` 的语法糖
    fn destroy(self) {
        // 解构 `self`
        let Pair(first, second) = self;

        println!("正在销毁 Pair({}, {})", first, second);

        // `first` 和 `second` 超出作用域并被释放
    }
}

#[wjj_lib::gen_test]
fn main() {
    let rectangle = Rectangle {
        // 使用双冒号调用关联函数
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 使用点运算符调用方法
    // 注意，第一个参数 `&self` 是隐式传递的
    // 即 `rectangle.perimeter()` 等同于 `Rectangle::perimeter(&rectangle)`
    println!("矩形周长：{}", rectangle.perimeter());
    println!("矩形面积：{}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 错误！`rectangle` 是不可变的，但这个方法需要可变对象
    //rectangle.translate(1.0, 0.0);
    // TODO ^ 尝试取消这行的注释

    // 正确！可变对象可以调用可变方法
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // 报错！之前的 `destroy` 调用已经"消耗"了 `pair`
    //pair.destroy();
    // TODO ^ 尝试取消注释这一行
}


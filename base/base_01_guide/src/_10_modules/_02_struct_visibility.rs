// 结构体可见性
// 结构体的字段具有额外的可见性级别。
// 字段默认为私有，可以使用 pub 修饰符来覆盖。
// 这种可见性只在从结构体定义模块外部访问时才有意义，其目的是实现信息隐藏（封装）。

mod my {
    // 一个具有泛型类型 `T` 公有字段的公有结构体
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 一个具有泛型类型 `T` 私有字段的公有结构体
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 一个公有的构造方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

#[wjj_lib::gen_test]
fn main() {
    // 具有公有字段的公有结构体可以正常构造
    let open_box = my::OpenBox { contents: "公开信息" };

    // 且可以正常访问其字段。
    println!("打开的盒子包含：{}", open_box.contents);

    // 具有私有字段的公有结构体不能使用字段名来构造。
    // 错误！`ClosedBox` 有私有字段
    //let closed_box = my::ClosedBox { contents: "机密信息" };
    // TODO ^ 尝试取消此行注释

    // 然而，具有私有字段的结构体可以使用公有构造函数创建
    let _closed_box = my::ClosedBox::new("机密信息");

    // 且无法访问公有结构体的私有字段。
    // 错误！`contents` 字段是私有的
    //println!("封闭的盒子包含：{}", _closed_box.contents);
    // TODO ^ 尝试取消此行注释
}


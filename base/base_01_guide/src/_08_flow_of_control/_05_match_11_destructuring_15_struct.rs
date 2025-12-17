// 结构体
// 同样，struct 可以按如下方式解构：

#[test]
fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // 尝试更改结构体中的值，看看会发生什么
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("x 的第一个元素是 1，b = {}，y = {}", b, y),

        // 你可以解构结构体并重命名变量，
        // 顺序并不重要
        Foo { y: 2, x: i } => println!("y 为 2，i = {:?}", i),

        // 你也可以忽略某些变量：
        Foo { y, .. } => println!("y = {}，我们不关心 x 的值", y),
        // 这会导致错误：模式中未提及字段 `x`
        //Foo { y } => println!("y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // 解构结构体不一定需要 match 块：
    let Foo { x: x0, y: y0 } = faa;
    println!("外部：x0 = {x0:?}，y0 = {y0}");

    // 解构也适用于嵌套结构体：
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar {
        foo: Foo {
            x: nested_x,
            y: nested_y,
        },
    } = bar;
    println!("嵌套：nested_x = {nested_x:?}，nested_y = {nested_y:?}");
}

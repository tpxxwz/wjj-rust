// 元组
// 元组是一个可以包含各种类型的值的集合。
// 元组使用圆括号 () 来构造，而且每个元组本身是一个类型标记为 (T1, T2, ...) 的值，其中 T1、T2 是其成员的类型。
// 函数可以使用元组来返回多个值，因为元组可以存储任意数量的值。

// 元组可以用作函数参数和返回值。
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` 可以用来把元组的成员绑定到变量。
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// 以下结构体将用于后续练习。
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

#[wjj_lib::gen_test]
fn main() {
    // 一个包含多种不同类型的元组。
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 可以使用元组索引来提取元组中的值。
    println!("长元组的第一个值：{}", long_tuple.0);
    println!("长元组的第二个值：{}", long_tuple.1);

    // 元组可以作为元组的成员。
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 元组是可打印的。
    println!("元组的元组：{:?}", tuple_of_tuples);

    // 但长元组（超过 12 个元素）无法打印。
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("过长的元组：{:?}", too_long_tuple);
    // TODO ^ 取消上面两行的注释以查看编译器错误

    let pair = (1, true);
    println!("对组是 {:?}", pair);

    println!("反转后的对组是 {:?}", reverse(pair));

    // 创建单元素元组时，需要使用逗号来区分它们
    // 与被括号包围的字面量。
    println!("单元素元组：{:?}", (5u32,));
    println!("仅是一个整数：{:?}", (5u32));

    // 可以通过解构元组来创建绑定。
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}、{:?}、{:?}、{:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}


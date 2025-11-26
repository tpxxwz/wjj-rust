// 别名
// 数据可以被不可变借用任意次数，但在不可变借用期间，原始数据不能被可变借用。
// 另一方面，同一时间只允许一个可变借用。只有在可变引用最后一次使用之后，原始数据才能再次被借用。

struct Point { x: i32, y: i32, z: i32 }

#[wjj_lib::gen_test]
fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // 可以通过引用和原始所有者访问数据
    println!("点的坐标为：({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // 错误！不能将 `point` 作为可变借用，因为它当前被不可变借用。
    // let mutable_borrow = &mut point;
    // TODO ^ 尝试取消注释此行

    // 这里再次使用了借用的值
    println!("点的坐标为：({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // 不可变引用在代码的剩余部分不再使用
    // 因此可以用可变引用重新借用。
    let mutable_borrow = &mut point;

    // 通过可变引用修改数据
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // 错误！不能将 `point` 作为不可变借用，因为它当前被可变借用。
    // let y = &point.y;
    // TODO ^ 尝试取消注释此行

    // 错误！无法打印，因为 `println!` 需要一个不可变引用。
    // println!("点的 Z 坐标是 {}", point.z);
    // TODO ^ 尝试取消注释此行

    // 正确！可变引用可以作为不可变引用传递给 `println!`
    println!("点的坐标为：({}, {}, {})",
                mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // 可变引用在代码的剩余部分不再使用，所以可以重新借用
    let new_borrowed_point = &point;
    println!("点现在的坐标为：({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}


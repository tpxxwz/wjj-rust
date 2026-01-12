// 数组/切片
// 与元组类似，数组和切片也可以用这种方式解构：

#[test]
fn main() {
    // 尝试改变数组中的值，或将其变成切片！
    let array = [1, -2, 6];

    match array {
        // 将第二个和第三个元素分别绑定到相应的变量
        [0, second, third] => println!("array[0] = 0，array[1] = {}，array[2] = {}", second, third),

        // 单个值可以用 _ 忽略
        [1, _, third] => println!("array[0] = 1，array[2] = {}，array[1] 被忽略了", third),

        // 你也可以绑定一部分值并忽略其余的
        [-1, second, ..] => println!("array[0] = -1，array[1] = {}，其他所有的都被忽略了", second),
        // 下面的代码无法编译
        // [-1, second] => ...

        // 或者将它们存储在另一个数组/切片中（类型取决于
        // 正在匹配的值的类型）
        [3, second, tail @ ..] => {
            println!("array[0] = 3，array[1] = {}，其他元素是 {:?}", second, tail)
        }

        // 结合这些模式，我们可以，例如，绑定第一个和
        // 最后一个值，并将其余的存储在一个单独的数组中
        [first, middle @ .., last] => println!(
            "array[0] = {}，中间部分 = {:?}，array[2] = {}",
            first, middle, last
        ),
    }
}

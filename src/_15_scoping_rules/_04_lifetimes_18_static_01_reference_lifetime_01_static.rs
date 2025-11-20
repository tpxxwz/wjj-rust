// 引用生命周期
// 作为引用生命周期，'static 表示该引用指向的数据在程序的整个剩余运行期间都有效。它仍然可以被强制转换为更短的生命周期。
//
// 有两种常见的方法可以创建具有 'static 生命周期的变量，它们都存储在二进制文件的只读内存中：
//
// 使用 static 声明创建一个常量。
// 创建一个字符串字面量，其类型为：&'static str。
// 请看下面的例子，展示了这些方法：

// 创建一个具有 `'static` 生命周期的常量。
static NUM: i32 = 18;

// 返回一个指向 `NUM` 的引用，其中 `'static` 生命周期
// 被强制转换为输入参数的生命周期。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // 创建一个字符串字面量并打印它：
        let static_string = "我存储在只读内存中";
        println!("static_string 的值：{}", static_string);

        // 当 `static_string` 离开作用域时，该引用
        // 不能再被使用，但数据仍然保留在二进制文件中。
    }

    {
        // 创建一个整数用于 `coerce_static` 函数：
        let lifetime_num = 9;

        // 将 `NUM` 的生命周期强制转换为与 `lifetime_num` 一致：
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM：{} 仍然可以访问！", NUM);
}

crate::gen_test!(main);

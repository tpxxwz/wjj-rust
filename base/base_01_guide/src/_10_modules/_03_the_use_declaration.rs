// use 声明
// use 声明可以将完整路径绑定到新名称，以便更轻松地访问。它通常这样使用：

// use crate::deeply::nested::{
//     my_first_function,
//     my_second_function,
//     AndATraitType
// };
//
// fn main() {
//     my_first_function();
// }

// 将 `deeply::nested::function` 路径绑定到 `other_function`。
use deeply::nested::function as other_function;

fn function() {
    println!("调用了 `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("调用了 `deeply::nested::function()`");
        }
    }
}

#[test]
fn main() {
    // 更方便地访问 `deeply::nested::function`
    other_function();

    println!("进入代码块");
    {
        // 这等同于 `use deeply::nested::function as function`。
        // 这个 `function()` 将遮蔽外部的同名函数。
        use deeply::nested::function;

        // `use` 绑定具有局部作用域。在这种情况下，
        // `function()` 的遮蔽仅在此代码块内有效。
        function();

        println!("离开代码块");
    }

    function();
}

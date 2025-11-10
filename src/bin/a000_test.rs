fn function() {
    println!("调用了 `function()`");
}

mod cool {
    pub fn function() {
        println!("调用了 `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("调用了 `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("调用了 `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // 让我们从这个作用域访问所有名为 `function` 的函数！
        print!("调用了 `my::indirect_call()`，它\n> ");

        // `self` 关键字指的是当前模块作用域 - 在这里是 `my`。
        // 调用 `self::function()` 和直接调用 `function()` 都会
        // 得到相同的结果，因为它们指向同一个函数。
        self::function();
        function();

        // 我们也可以使用 `self` 来访问 `my` 内的另一个模块：
        self::cool::function();

        // `super` 关键字指的是父作用域（`my` 模块之外）。
        super::function();

        // 这将绑定到 *crate* 作用域中的 `cool::function`。
        // 在这种情况下，crate 作用域是最外层作用域。
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
// 创建库
// 让我们创建一个库，然后看看如何将它链接到另一个 crate。
//
// 在 _01_creating_a_library.rs 文件中：
pub fn public_function() {
    println!("调用了 rary 的 `public_function()`");
}

fn private_function() {
    println!("调用了 rary 的 `private_function()`");
}

pub fn indirect_access() {
    print!("调用了 rary 的 `indirect_access()`，它\n> ");

    private_function();
}

// $ rustc --crate-type=lib _01_creating_a_library.rs --crate-name rary
// $ ls lib*
// library.rlib

// 库文件名会自动添加 "lib" 前缀，默认使用 crate 文件的名称。
// 但可以通过向 rustc 传递 --crate-name 选项或使用 crate_name 属性 来覆盖这个默认名称。

// Crates
// crate_type 属性可用于告诉编译器一个 crate 是二进制文件还是库（甚至是哪种类型的库），
// 而 crate_name 属性可用于设置 crate 的名称。
//
// 然而，需要注意的是，当使用 Rust 的包管理器 Cargo 时，crate_type 和 crate_name 属性完全不起作用。
// 由于大多数 Rust 项目都使用 Cargo，这意味着 crate_type 和 crate_name 在实际使用中的应用相对有限。

// 这个 crate 是一个库
#![crate_type = "lib"]
// 这个库的名称是 "rary"
#![crate_name = "rary"]

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

// 当使用 crate_type 属性时，我们就不再需要向 rustc 传递 --crate-type 标志。
// $ rustc lib.rs
// $ ls lib*
// library.rlib

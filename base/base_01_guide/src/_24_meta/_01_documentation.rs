// 文档
// 使用 cargo doc 在 target/doc 目录下构建文档。运行 cargo doc --open 将自动在浏览器中打开文档。
//
// 使用 cargo test 运行所有测试（包括文档测试）。如果只想运行文档测试，请使用 cargo test --doc。
//
// 这些命令会根据需要适当地调用 rustdoc（和 rustc）。

// 文档注释
// 文档注释对需要文档的大型项目非常有用。运行 rustdoc 时，这些注释会被编译成文档。文档注释以 /// 开头，并支持 Markdown 语法。
#![crate_name = "doc"]

/// 这里表示一个人类
pub struct Person {
    /// 一个人必须有名字，不管朱丽叶有多么讨厌这一点
    name: String,
}

impl Person {
    /// 创建一个具有给定名字的人。
    ///
    /// # 示例
    ///
    /// ```
    /// // 你可以在注释中的代码块里编写 Rust 代码
    /// // 如果向 `rustdoc` 传递 --test 参数，它甚至会为你测试这段代码！
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// 给出友好的问候！
    ///
    /// 对调用此方法的 `Person` 说 "Hello, [name](Person::name)"。
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

#[wjj_lib::gen_test]
fn main() {
    let john = Person::new("John");

    john.hello();
}

// 要运行测试，首先将代码构建为库，然后告诉 rustdoc 库的位置，以便它可以将库链接到每个文档测试程序中：

// $ rustc doc.rs --crate-type lib
// $ rustdoc --test --extern doc="libdoc.rlib" doc.rs

// 文档属性
// 以下是几个与 rustdoc 配合使用的最常见 #[doc] 属性示例。

// inline
// 用于内联文档，而非链接到单独的页面。

#[doc(inline)]
pub use bar::Bar;

/// bar 的文档
pub mod bar {
    /// Bar 的文档
    pub struct Bar;
}

// no_inline
// 用于防止链接到单独页面或其他任何地方。

// libcore/prelude 中的示例
#[doc(no_inline)]
pub use std::mem::drop;
// pub use core::mem::drop;

// hidden
// 使用此属性告诉 rustdoc 不要在文档中包含此内容

// futures-rs 库中的示例
// #[doc(hidden)]
// pub use self::async_await::*;

// 在文档生成方面，rustdoc 被社区广泛使用。它是用来生成 标准库文档 的工具。


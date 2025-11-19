// 依赖
// 大多数程序都依赖于一些库。如果你曾经手动管理过依赖，你就知道这有多么痛苦。
// 幸运的是，Rust 生态系统标配了 cargo！cargo 可以为项目管理依赖。
//
// 要创建一个新的 Rust 项目，可以使用以下命令：
//
// # 创建二进制项目
// cargo new foo
//
// # 创建库项目
// cargo new --lib bar
//
// 在本章的剩余部分，我们假设我们正在创建一个二进制项目，而不是一个库项目，但所有的概念都是相同的。
//
// 执行上述命令后，你应该会看到如下的文件结构：
// .
// ├── bar
// │   ├── Cargo.toml
// │   └── src
// │       └── lib.rs
// └── foo
// ├── Cargo.toml
// └── src
// └── main.rs
//
// main.rs 是你新建的 foo 项目的主源文件 —— 这一点没什么新鲜的。
// Cargo.toml 则是这个项目的 cargo 配置文件。如果你查看它的内容，应该会看到类似这样的内容：
// [package]
// name = "foo"
// version = "0.1.0"
// authors = ["mark"]
//
// [dependencies]
//
// [package] 下的 name 字段决定了项目的名称。
// 如果你将 crate 发布到 crates.io（稍后会详细介绍），这个名称将被使用。
// 同时，它也是编译时生成的二进制文件的名称。
//
// version 字段是使用语义化版本控制的 crate 版本号。
//
// authors 字段是发布 crate 时使用的作者列表。
//
// [dependencies] 部分允许你为项目添加依赖。
//
// 举个例子，假设我们想让程序拥有一个出色的命令行界面（CLI）。
// 你可以在 crates.io（Rust 官方包注册中心）上找到许多优秀的包。
// 其中，clap 是一个广受欢迎的选择。
// 在撰写本文时，clap 的最新发布版本是 2.27.1。
// 要在我们的程序中添加这个依赖，只需在 Cargo.toml 的 [dependencies] 下添加：
// clap = "2.27.1"。就这么简单！现在你就可以在程序中使用 clap 了。
//
// cargo 还支持其他类型的依赖。这里给出一个简单的示例：
//
// [package]
// name = "foo"
// version = "0.1.0"
// authors = ["mark"]
//
// [dependencies]
// clap = "2.27.1" # 来自 crates.io
// rand = { git = "https://github.com/rust-lang-nursery/rand" } # 来自在线仓库
// bar = { path = "../bar" } # 来自本地文件系统的路径
// cargo 不仅仅是一个依赖管理器。Cargo.toml 的格式规范中列出了所有可用的配置选项。
//
// 我们可以在项目目录的任何位置（包括子目录！）执行 cargo build 来构建项目。
// 也可以使用 cargo run 来构建并运行。
// 请注意，这些命令会解析所有依赖，必要时下载 crate，并构建所有内容，包括你的 crate。
// （值得一提的是，它只会重新构建尚未构建的部分，类似于 make）。
//
// 瞧！就是这么简单！
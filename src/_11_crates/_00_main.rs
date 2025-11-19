// Crates
// 在 Rust中，crate 是一个编译单元。
// 每当执行 rustc some_file.rs 时，some_file.rs 就被视为 crate文件。
// 如果 some_file.rs 中包含 mod 声明，那么模块文件的内容会在编译器处理之前被插入到 crate 文件中 mod 声明的位置。
// 换句话说，模块不会被单独编译，只有 crate 才会被编译。
//
// 一个 crate 可以被编译成二进制文件或库。
// 默认情况下，rustc 会将 crate 编译成二进制文件。
// 通过向 rustc 传递 --crate-type 标志并指定 lib，可以改变这一默认行为。
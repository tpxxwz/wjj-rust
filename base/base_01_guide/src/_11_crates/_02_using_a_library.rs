// 使用库
// 要将 crate 链接到这个新库，可以使用 rustc 的 --extern 标志。
// 库中的所有项目都会被导入到一个与库同名的模块下。这个模块的行为通常与其他模块相同。

// extern crate rary; // Rust 2015 版本或更早版本可能需要此声明

#[wjj_lib::gen_test]
fn main() {
    // 下一行的注释解开 用rustc --extern的命令执行
    // rary::public_function();

    // 错误！`private_function` 是私有的
    //rary::private_function();

    // 下一行的注释解开 用rustc --extern的命令执行
    // rary::indirect_access();
}



// # Where library.rlib is the path to the compiled library, assumed that it's
// # in the same directory here:
// $ rustc _02_using_a_library.rs --extern rary=library.rlib && ./_02_using_a_library
// called rary's `public_function()`
// called rary's `indirect_access()`, that
// > called rary's `private_function()`

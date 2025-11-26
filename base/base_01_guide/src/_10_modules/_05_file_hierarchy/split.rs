// 在 split.rs 文件中：

// 这个声明会查找名为 `my.rs` 的文件，并将其内容
// 插入到当前作用域下名为 `my` 的模块中
use super::my;
// use crate::_10_modules::_05_file_hierarchy::my;

fn function() {
    println!("调用了 `function()`");
}

#[wjj_lib::gen_test]
fn main() {
    // super::my::function();
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}


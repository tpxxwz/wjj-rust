// 在 my/inaccessible.rs 文件中：
#[allow(dead_code)]
pub fn public_function() {
    println!("调用了 `my::inaccessible::public_function()`");
}
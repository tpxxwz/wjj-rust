// let-else
// 🛈 自 Rust 1.65 版本起稳定
// 🛈 你可以通过这种方式编译来指定特定版本：rustc --edition=2021 main.rs
//
// let-else 语法允许可能失败的模式匹配像普通 let 一样绑定变量到当前作用域，或在匹配失败时执行中断操作（如 break、return、panic!）。

use std::str::FromStr;

fn get_count_item_old(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("无法分割计数项对：'{s}'"),
    };
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("无法解析整数：'{count_str}'");
    };
    (count, item)
}

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("无法分割计数项对：'{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("无法解析整数：'{count_str}'");
    };
    (count, item)
}

#[test]
fn main() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}

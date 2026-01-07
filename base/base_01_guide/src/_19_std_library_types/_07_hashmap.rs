// HashMap
// 向量（Vector）通过整数索引存储值，而 HashMap 则通过键存储值。
// HashMap 的键可以是布尔值、整数、字符串，或任何其他实现了 Eq 和 Hash trait 的类型。下一节将详细介绍这一点。
//
// 与向量类似，HashMap 也可以增长，但当有多余空间时，HashMap 还能自动收缩。
// 你可以使用 HashMap::with_capacity(uint) 创建一个具有指定初始容量的 HashMap，
// 或使用 HashMap::new() 来获得一个具有默认初始容量的 HashMap（推荐）。

use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => {
            "很抱歉，无法接通您拨打的电话。
            请挂机后重试。"
        }
        "645-7689" => {
            "您好，这里是 Awesome 先生的披萨店。我是 Fred。
            请问今天您想点些什么？"
        }
        _ => "嗨！请问是哪位？",
    }
}

#[test]
fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // 接受一个引用并返回 Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("正在呼叫 Daniel：{}", call(number)),
        _ => println!("没有 Daniel 的电话号码。"),
    }

    // 如果插入的值是新的，`HashMap::insert()` 返回 `None`
    // 否则返回 `Some(value)`
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("正在呼叫 Ashley：{}", call(number)),
        _ => println!("没有 Ashley 的电话号码。"),
    }

    contacts.remove(&"Ashley");

    // `HashMap::iter()` 返回一个迭代器，该迭代器以任意顺序生成
    // (&'a key, &'a value) 键值对。
    for (contact, &number) in contacts.iter() {
        println!("正在呼叫{}：{}", contact, call(number));
    }
}

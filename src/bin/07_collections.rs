// 07_collections.rs
// 常见集合及操作

fn main() {
    // Vector
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("第一个元素: {}", v[0]);

    // 遍历Vector
    for i in &v {
        println!("{}", i);
    }

    // String
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // HashMap
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Blue队的分数: {:?}", scores.get("Blue"));

    // 遍历HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
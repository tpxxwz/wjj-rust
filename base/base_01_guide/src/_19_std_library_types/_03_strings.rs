// 字符串
// Rust 中最常用的两种字符串类型是 String 和 &str。
//
// String 存储为字节向量（Vec<u8>），但保证始终是有效的 UTF-8 序列。String 在堆上分配，可增长，且不以 null 结尾。
//
// &str 是一个切片（&[u8]），它始终指向一个有效的 UTF-8 序列，可以用来查看 String，就像 &[T] 是 Vec<T> 的一个视图一样。

#[wjj_lib::gen_test]
fn main() {
    // （所有的类型注解都不是必须的）
    // 一个指向只读内存中分配的字符串的引用
    let pangram: &'static str = "敏捷的棕色狐狸跳过懒惰的狗";
    println!("全字母句：{}", pangram);

    // 反向遍历单词，不会分配新的字符串
    println!("单词逆序");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // 将字符复制到向量中，排序并去重
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // 创建一个空的、可增长的 `String`
    let mut string = String::new();
    for c in chars {
        // 在字符串末尾插入一个字符
        string.push(c);
        // 在字符串末尾追加另一个字符串
        string.push_str(", ");
    }

    // 修剪后的字符串是原始字符串的一个切片，因此不会进行新的内存分配
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("使用的字符：{}", trimmed_str);

    // 在堆上分配一个字符串
    let alice = String::from("我喜欢狗");
    // 分配新的内存并在其中存储修改后的字符串
    let bob: String = alice.replace("狗", "猫");

    println!("爱丽丝说：{}", alice);
    println!("鲍勃说：{}", bob);
}


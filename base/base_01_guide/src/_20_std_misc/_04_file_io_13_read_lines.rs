// read_lines
// 一种简单的方法
// 对于初学者来说，这可能是从文件中读取行的一个合理的初步尝试。

use std::fs::read_to_string;

fn read_lines1(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
// 由于 lines() 方法返回文件中各行的迭代器，我们可以内联执行 map 并收集结果，从而得到一个更简洁流畅的表达式。

fn read_lines2(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // 遇到可能的文件读取错误时 panic
        .lines()  // 将字符串分割成字符串切片的迭代器
        .map(String::from)  // 将每个切片转换为字符串
        .collect()  // 将它们收集到一个向量中
}
// 注意，在上述两个示例中，我们都必须将 lines() 返回的 &str 引用转换为拥有所有权的 String 类型，
// 分别使用 .to_string() 和 String::from。

// 一种更高效的方法
// 在这里，我们将打开的 File 的所有权传递给 BufReader 结构体。BufReader 使用内部缓冲区来减少中间分配。
//
// 我们还对 read_lines 函数进行了改进，使其返回一个迭代器，而不是为每行内容在内存中分配新的 String 对象。

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[wjj_lib::gen_test]
fn main() {
    // 文件 hosts.txt 必须存在于当前路径下
    if let Ok(lines) = read_lines("./hosts.txt") {
        // 消耗迭代器，返回一个（可选的）String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}

// 输出被包装在 Result 中以便于错误匹配。
// 返回一个指向文件行读取器的迭代器。
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



// 运行此程序将逐行打印文件内容。
//
// $ echo -e "127.0.0.1\n192.168.0.1\n" > hosts.txt
// $ rustc read_lines.rs && ./read_lines
// 127.0.0.1
// 192.168.0.1
// （注意，由于 File::open 需要一个泛型 AsRef<Path> 作为参数，
// 我们使用 where 关键字为 read_lines() 方法定义了相同的泛型约束。）
//
// 这种方法比在内存中创建包含整个文件内容的 String 更加高效。特别是在处理大文件时，后者可能会导致性能问题。
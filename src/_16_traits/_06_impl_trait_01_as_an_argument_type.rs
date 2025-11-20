// 作为参数类型
// 如果你的函数对某个 trait 是泛型的，但不关心具体类型，你可以使用 impl Trait 作为参数类型来简化函数声明。
//
// 例如，考虑以下代码：

fn parse_csv_document1<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // 遍历数据源中的每一行
            line.map(|line| {
                // 如果成功读取行，则处理它；否则，返回错误
                line.split(',') // 按逗号分割行
                    .map(|entry| String::from(entry.trim())) // 去除首尾空白
                    .collect() // 将该行的所有字符串收集到 Vec<String> 中
            })
        })
        .collect() // 将所有行收集到 Vec<Vec<String>> 中
}

// parse_csv_document 是泛型函数，可以接受任何实现了 BufRead 的类型，如 BufReader<File> 或 [u8]。
// 但具体的 R 类型并不重要，R 仅用于声明 src 的类型。因此，这个函数也可以写成：

fn parse_csv_document2(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // 遍历数据源中的每一行
            line.map(|line| {
                // 如果成功读取行，则处理它；否则，返回错误
                line.split(',') // 按逗号分割行
                    .map(|entry| String::from(entry.trim())) // 去除首尾空白
                    .collect() // 将该行的所有字符串收集到 Vec<String> 中
            })
        })
        .collect() // 将所有行收集到 Vec<Vec<String>> 中
}
// 注意，使用 impl Trait 作为参数类型意味着你无法显式指定使用的函数形式。
// 例如，parse_csv_document::<std::io::Empty>(std::io::empty()) 在第二个例子中将无法工作。
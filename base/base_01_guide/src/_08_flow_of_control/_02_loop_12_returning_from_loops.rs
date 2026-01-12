// 在 loop 中返回值
// loop 的一个用途是重试操作直到成功。
// 如果操作返回一个值，你可能需要将它传递给代码的其余部分：将它放在 break 之后，它将被 loop 表达式返回。

#[test]
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

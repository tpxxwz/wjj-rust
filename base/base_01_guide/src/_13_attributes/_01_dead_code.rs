// dead_code
// 编译器提供了一个 dead_code lint，用于警告未使用的函数。可以使用属性来禁用这个 lint。
fn used_function() {}

// `#[allow(dead_code)]` 是一个用于禁用 `dead_code` lint 的属性
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ 添加一个属性来抑制警告

#[test]
fn main() {
    used_function();
}
// 注意，在实际程序中，你应该消除无用代码。
// 在这些示例中，我们会在某些地方允许存在无用代码，这是因为这些示例具有交互性质。

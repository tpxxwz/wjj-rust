// if let
// 在某些情况下，使用 match 匹配枚举可能会显得繁琐。例如：

// 创建 `Option<i32>` 类型的 `optional`
// let optional = Some(7);
//
// match optional {
// Some(i) => println!("这是一个很长的字符串，其中包含 `{:?}`", i),
// _ => {},
// // ^ 这是必需的，因为 `match` 要求穷举所有情况。
// // 是不是觉得有些浪费空间？
// };

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

#[wjj_lib::gen_test]
fn main() {
    // 对于这种情况，if let 更加简洁，而且还允许指定各种失败时的处理选项：

    // 以下都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构的含义是：如果 `let` 能将 `number` 解构为
    // `Some(i)`，则执行代码块（`{}`）。
    if let Some(i) = number {
        println!("匹配到 {:?}！", i);
    }

    // 如果需要指定匹配失败的情况，可以使用 else：
    if let Some(i) = letter {
        println!("匹配到 {:?}！", i);
    } else {
        // 解构失败。转到失败处理的情况。
        println!("没有匹配到数字。那就用一个字母吧！");
    }

    // 提供一个修改后的失败条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("匹配到 {:?}！", i);
    // 解构失败。评估 `else if` 条件，看是否应该执行替代的失败分支：
    } else if i_like_letters {
        println!("没有匹配到数字。那就用一个字母吧！");
    } else {
        // 条件判断为假。这个分支是默认情况：
        println!("我不喜欢字母。那就用个表情符号吧 :)！");
    }

    // 同样地，if let 可以用来匹配任何枚举值
    // 创建示例变量
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // 变量 a 匹配 Foo::Bar
    if let Foo::Bar = a {
        println!("a 是 foobar");
    }

    // 变量 b 不匹配 Foo::Bar
    // 所以这里不会打印任何内容
    if let Foo::Bar = b {
        println!("b 是 foobar");
    }

    // 变量 c 匹配 Foo::Qux，它包含一个值
    // 类似于前面例子中的 Some()
    if let Foo::Qux(value) = c {
        println!("c 是 {}", value);
    }

    // `if let` 也可以进行绑定
    if let Foo::Qux(value @ 100) = c {
        println!("c 是一百");
    }

    // if let 的另一个优点是它允许我们匹配非参数化的枚举变体。
    // 即使在枚举没有实现或派生 PartialEq 的情况下也是如此。
    // 在这种情况下，if Foo::Bar == a 将无法编译，因为枚举的实例无法进行相等比较，但 if let 仍然可以正常工作。
    //
    // 想要挑战一下吗？请修改以下示例，使用 if let：
    let a = Foo::Bar;

    // 变量 a 匹配 Foo::Bar
    // if Foo::Bar == a {
        // ^-- 这会导致编译时错误。请改用 `if let`。
        println!("a 是 foobar");
    // }
}




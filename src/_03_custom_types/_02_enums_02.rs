// 类型别名
// 使用类型别名可以通过别名引用每个枚举变体。当枚举名称过长或过于泛化，而你想重命名它时，这会很有用。

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// 创建类型别名
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // 我们可以通过别名引用每个变体，而不是使用又长又不便的名称。
    let x = Operations::Add;
}

// 你最常见到这种用法的地方是在使用 Self 别名的 impl 块中。

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
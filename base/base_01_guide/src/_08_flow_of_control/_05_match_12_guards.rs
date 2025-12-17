// 守卫
// match 分支可以使用守卫进行额外的筛选。

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

#[test]
fn main() {
    let temperature = Temperature::Celsius(35);
    // ^ TODO：尝试为 `temperature` 赋予不同的值

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}°C 高于 30°C", t),
        // `if condition` 部分 ^ 就是守卫
        Temperature::Celsius(t) => println!("{}°C 不高于 30°C", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}°F 高于 86°F", t),
        Temperature::Fahrenheit(t) => println!("{}°F 不高于 86°F", t),
    }

    // 注意，编译器在检查 match 表达式是否涵盖了所有模式时，不会考虑守卫条件。
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("零"),
        i if i > 0 => println!("大于零"),
        _ => unreachable!("不应该发生。"),
        // TODO ^ 取消注释以修复编译错误
    }
}

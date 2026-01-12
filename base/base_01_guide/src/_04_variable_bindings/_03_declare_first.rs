// 先声明
// 可以先声明变量绑定然后再初始化，但所有变量绑定在使用前必须先初始化：编译器禁止使用未初始化的变量绑定，因为这会导致未定义行为。
//
// 在函数中先声明变量绑定而稍后再初始化的做法并不常见。当初始化与声明分离时，读者更难找到初始化的位置。
// 更常见的做法是在变量即将使用的地方附近声明并初始化变量绑定。

#[test]
fn main() {
    // 声明一个变量绑定
    let a_binding;

    {
        let x = 2;

        // 初始化绑定
        a_binding = x * x;
    }

    println!("绑定：{}", a_binding);

    let another_binding;

    // 错误！使用未初始化的绑定
    // println!("另一个绑定：{}", another_binding);
    // 修复：^ 注释掉此行

    another_binding = 1;

    println!("另一个绑定：{}", another_binding);
}

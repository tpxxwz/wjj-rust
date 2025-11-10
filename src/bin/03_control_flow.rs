// 03_control_flow.rs
// 流程控制

fn main() {
    // if表达式
    let number = 7;
    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }

    // if-else if
    let number = 6;
    if number % 4 == 0 {
        println!("能被4整除");
    } else if number % 3 == 0 {
        println!("能被3整除");
    } else {
        println!("不能被4或3整除");
    }

    // 在let语句中使用if
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number的值为: {}", number);

    // 循环
    // loop循环
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }
    println!("loop循环结束，count = {}", count);

    // while循环
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("while循环结束");

    // for循环
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("元素值为: {}", element);
    }

    // Range与for循环
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("for循环结束");
}
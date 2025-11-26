// 嵌套和标签
// 在处理嵌套循环时，可以 break 或 continue 外层循环。
// 这种情况下，循环必须用 'label 标记，并且必须将标签传递给 break/continue 语句。

#![allow(unreachable_code, unused_labels)]
#[wjj_lib::gen_test]
fn main() {
    'outer: loop {
        println!("进入外层循环");

        'inner: loop {
            println!("进入内层循环");

            // 这只会中断内层循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("这一点永远不会到达");
    }

    println!("退出外层循环");
}



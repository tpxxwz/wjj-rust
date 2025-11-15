// 此外，Rust 允许变量遮蔽。

fn main() {
    let shadowed_binding = 1;

    {
        println!("被遮蔽前：{}", shadowed_binding);

        // 这个绑定*遮蔽*了外部的绑定
        let shadowed_binding = "abc";

        println!("内部代码块中被遮蔽：{}", shadowed_binding);
    }
    println!("内部代码块外：{}", shadowed_binding);

    // 这个绑定*遮蔽*了之前的绑定
    let shadowed_binding = 2;
    println!("外部代码块中被遮蔽：{}", shadowed_binding);
}
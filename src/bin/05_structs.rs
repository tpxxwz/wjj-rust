// 05_structs.rs
// 结构体

// 定义结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 创建结构体实例
    let user1 = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("用户名: {}, 邮箱: {}", user1.username, user1.email);

    // 可变结构体实例
    let mut user2 = User {
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        sign_in_count: 1,
        active: true,
    };
    user2.email = String::from("new_email@example.com");
    println!("新邮箱: {}", user2.email);

    // 结构体更新语法
    let user3 = User {
        username: String::from("user3"),
        email: String::from("user3@example.com"),
        ..user2 // 使用user2的其他字段值
    };
    println!("用户3的登录次数: {}", user3.sign_in_count);

    // 元组结构体
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("黑色RGB值: ({}, {}, {})", black.0, black.1, black.2);

    // 单元结构体
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
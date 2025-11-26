// 更改或自定义键类型
// 任何实现了 Eq 和 Hash trait 的类型都可以作为 HashMap 的键。这包括：
//
// bool（虽然用处不大，因为只有两个可能的键值）
// int、uint 及其所有变体
// String 和 &str（专业提示：你可以使用 String 作为 HashMap 的键，并用 &str 调用 .get() 方法）
// 注意，f32 和 f64 并未实现 Hash trait，这很可能是因为浮点数精度误差会导致将它们用作哈希映射的键时极易出错。
//
// 如果集合类中包含的类型分别实现了 Eq 和 Hash，那么这些集合类也会实现 Eq 和 Hash。
// 例如，如果 T 实现了 Hash，那么 Vec<T> 也会实现 Hash。
//
// 你可以通过一行代码轻松地为自定义类型实现 Eq 和 Hash：#[derive(PartialEq, Eq, Hash)]
//
// 编译器会完成剩余的工作。如果你想对细节有更多控制，可以自己实现 Eq 和/或 Hash。
// 本指南不会涉及实现 Hash 的具体细节。
//
// 为了尝试在 HashMap 中使用 struct，让我们来创建一个非常简单的用户登录系统：

use std::collections::HashMap;

// Eq 要求你在类型上派生 PartialEq。
#[derive(PartialEq, Eq, Hash)]
struct Account<'a>{
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a>{
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>,
                 username: &'a str, password: &'a str){
    println!("用户名：{}", username);
    println!("密码：{}", password);
    println!("正在尝试登录...");

    let logon = Account {
        username,
        password,
    };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("登录成功！");
            println!("姓名：{}", account_info.name);
            println!("电子邮箱：{}", account_info.email);
        },
        _ => println!("登录失败！"),
    }
}

#[wjj_lib::gen_test]
fn main(){
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "psasword123");

    try_logon(&accounts, "j.everyman", "password123");
}


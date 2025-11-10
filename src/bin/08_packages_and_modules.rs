// 08_packages_and_modules.rs
// 包与模块

// 定义模块
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("已加入等待列表");
        }
    }
}

// 使用模块
use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();

    // 使用绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 使用相对路径
    front_of_house::hosting::add_to_waitlist();
}
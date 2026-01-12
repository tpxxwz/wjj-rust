// use
// use 声明可以用来避免手动作用域限定：

// 这个属性用于隐藏未使用代码的警告。
#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

#[test]
fn main() {
    // 显式 `use` 每个名称，使它们可以不需要
    // 手动作用域限定就能使用。
    // use crate::_03_custom_types::_02_enums_11_use::Stage::{Advanced, Beginner};
    use Stage::{Advanced, Beginner};
    // // 自动 `use` `Role` 内的每个名称。
    // use crate::_03_custom_types::_02_enums_11_use::Role::*;
    use Role::*;

    // 等同于 `Stage::Beginner`。
    let stage = Beginner;
    // 等同于 `Role::Student`。
    let role = Student;

    match stage {
        // 注意由于上面的显式 `use`，这里不需要作用域限定。
        Beginner => println!("初学者正在开始他们的学习之旅！"),
        Advanced => println!("高级学习者正在掌握他们的科目..."),
    }

    match role {
        // 再次注意这里不需要作用域限定。
        Student => println!("学生正在获取知识！"),
        Teacher => println!("教师正在传播知识！"),
    }
}

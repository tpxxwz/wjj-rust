// 父特质
// Rust 没有“继承”，但你可以将一个 trait 定义为另一个 trait 的超集。例如：

trait Person {
    fn name(&self) -> String;
}

// Person 是 Student 的超 trait。
// 实现 Student 需要你同时实现 Person。
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent（计算机科学学生）是 Programmer 和 Student 的子 trait。
// 实现 CompSciStudent 需要你实现两个超 trait。
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "我的名字是 {}，我就读于 {}。我最喜欢的语言是 {}。我的 Git 用户名是 {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

#[wjj_lib::gen_test]
fn main() {}


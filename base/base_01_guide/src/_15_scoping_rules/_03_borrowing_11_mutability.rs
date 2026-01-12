// 可变性
// 可变数据可以使用 &mut T 进行可变借用。
// 这被称为可变引用，并给予借用者读写访问权。
// 相比之下，&T 通过不可变引用借用数据，借用者可以读取数据但不能修改它：

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` 是对分配在只读内存中的字符串的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}

// 此函数接受一个对 Book 类型的引用
fn borrow_book(book: &Book) {
    println!("我不可变地借用了《{}》- {} 版", book.title, book.year);
}

// 此函数接受一个对可变 Book 的引用，并将 `year` 改为 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("我可变地借用了《{}》- {} 版", book.title, book.year);
}

#[test]
fn main() {
    // 创建一个名为 `immutabook` 的不可变 Book 实例
    let immutabook = Book {
        // 字符串字面量的类型是 `&'static str`
        author: "Douglas Hofstadter",
        title: "哥德尔、埃舍尔、巴赫",
        year: 1979,
    };

    // 创建 `immutabook` 的可变副本，并命名为 `mutabook`
    let mut mutabook = immutabook;

    // 不可变地借用一个不可变对象
    borrow_book(&immutabook);

    // 不可变地借用一个可变对象
    borrow_book(&mutabook);

    // 可变地借用一个可变对象
    new_edition(&mut mutabook);

    // 错误！不能将不可变对象作为可变对象借用
    // new_edition(&mut immutabook);
    // 修复：^ 注释掉此行
}

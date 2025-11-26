// 实现
// 与函数类似，实现（impl）在涉及泛型时也需要谨慎处理。

struct S; // 具体类型 `S`
struct GenericVal<T>(T); // 泛型类型 `GenericVal`

// GenericVal 的实现，这里我们显式指定类型参数：
impl GenericVal<f32> {} // 指定 `f32`
impl GenericVal<S> {} // 指定上面定义的 `S`

// `<T>` 必须放在类型前面以保持泛型
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// Val 的实现
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// 为泛型类型 `T` 实现 GenVal
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

#[wjj_lib::gen_test]
fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}


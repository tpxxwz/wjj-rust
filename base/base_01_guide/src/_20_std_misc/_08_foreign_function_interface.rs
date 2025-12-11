// 外部函数接口
// Rust 提供了与 C 库交互的外部函数接口（FFI）。
// 外部函数必须在 extern 块内声明，并使用 #[link] 属性标注外部库的名称。

use std::fmt;

// 此 extern 块链接到 libm 库
#[cfg(target_family = "windows")]
#[link(name = "msvcrt")]
unsafe extern "C" {
    // 这是一个外部函数
    // 用于计算单精度复数的平方根
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}
#[cfg(target_family = "unix")]
#[link(name = "m")]
unsafe extern "C" {
    // 这是一个外部函数
    // 用于计算单精度复数的平方根
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}

// 由于调用外部函数被认为是不安全的，
// 通常会为它们编写安全的包装函数。
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

#[test]
fn main() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // 调用外部函数是一个不安全的操作
    let z_sqrt = unsafe { csqrtf(z) };

    println!("{:?} 的平方根是 {:?}", z, z_sqrt);

    // 调用封装了不安全操作的安全 API
    println!("cos({:?}) = {:?}", z, cos(z));
}

// 单精度复数的最小实现
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

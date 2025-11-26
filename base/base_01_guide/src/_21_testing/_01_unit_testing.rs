// 单元测试
// 测试是 Rust 函数，用于验证非测试代码是否按预期方式运行。
// 测试函数的主体通常包括一些准备工作，运行待测试的代码，然后断言结果是否符合预期。
//
// 大多数单元测试都放在带有 #[cfg(test)] 属性的 tests 模块中。测试函数用 #[test] 属性标记。
//
// 当测试函数中出现恐慌（panic）时，测试就会失败。以下是一些辅助宏：
//
// assert!(expression) - 如果表达式求值为 false，则会触发 panic。
// assert_eq!(left, right) 和 assert_ne!(left, right) - 分别用于测试左右表达式的相等性和不相等性。

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 这是一个非常糟糕的加法函数，它的目的是在这个例子中失败。
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests1 {
    // 注意这个有用的惯用法：从外部作用域（对于 mod tests 而言）导入名称。
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // 这个断言会触发，测试将失败。
        // 请注意，私有函数也可以被测试！
        assert_eq!(bad_add(1, 2), 3);
    }
}

// 可以使用 cargo test 命令运行测试。
//
// $ cargo test
//
// running 2 tests
// test tests::test_bad_add ... FAILED
// test tests::test_add ... ok
//
// failures:
//
// ---- tests::test_bad_add stdout ----
// thread 'tests::test_bad_add' panicked at 'assertion failed: `(left == right)`
// left: `-1`,
// right: `3`', src/lib.rs:21:8
// note: Run with `RUST_BACKTRACE=1` for a backtrace.
//
//
// failures:
// tests::test_bad_add
//
// test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

// 测试与 ? 运算符
// 之前的单元测试示例都没有返回类型。但在 Rust 2018 版本中，
// 你的单元测试可以返回 Result<()>，这使得你可以在测试中使用 ? 运算符！这可以使测试代码更加简洁。
fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("负浮点数没有平方根".to_owned())
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}

// 测试 panic
// 要检查在某些情况下应该触发恐慌的函数，可以使用 #[should_panic] 属性。
// 这个属性接受可选参数 expected = ，用于指定预期的恐慌消息文本。
// 如果你的函数可能以多种方式触发 panic，这有助于确保你的测试正在检查正确的 panic 情况。
//
// 注意：Rust 还允许使用简写形式 #[should_panic = "message"]，
// 其功能与 #[should_panic(expected = "message")] 完全相同。
// 两种形式都有效；后者更常用，被认为更明确。
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("除以零错误");
    } else if a < b {
        panic!("除法结果为零");
    }
    a / b
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "除法结果为零")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }

    #[test]
    #[should_panic = "除法结果为零"] // 这样也可以
    fn test_specific_panic_shorthand() {
        divide_non_zero_result(1, 10);
    }
}

// 运行这些测试会得到以下结果：
// $ cargo test
//
// running 3 tests
// test tests::test_any_panic ... ok
// test tests::test_divide ... ok
// test tests::test_specific_panic ... ok
// test tests::test_specific_panic_shorthand ... ok
//
// test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//
// Doc-tests tmp-test-should-panic
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

// 运行特定测试
// 要运行特定的测试，可以在 cargo test 命令中指定测试名称。
//
// $ cargo test test_any_panic
// running 1 test
// test tests::test_any_panic ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
//
// Doc-tests tmp-test-should-panic
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
// 要运行多个测试，可以指定测试名称的一部分，该部分匹配所有应该运行的测试。
// $ cargo test panic
// running 2 tests
// test tests::test_any_panic ... ok
// test tests::test_specific_panic ... ok
//
// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
//
// Doc-tests tmp-test-should-panic
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

// 忽略测试
// 可以使用 #[ignore] 属性标记测试以排除某些测试。或者使用命令 cargo test -- --ignored 来运行这些被忽略的测试。

#[cfg(test)]
mod tests4 {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_hundred() {
        assert_eq!(add(100, 2), 102);
        assert_eq!(add(2, 100), 102);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }
}

// $ cargo test
// running 3 tests
// test tests::ignored_test ... ignored
// test tests::test_add ... ok
// test tests::test_add_hundred ... ok
//
// test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
//
// Doc-tests tmp-ignore
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//
// $ cargo test -- --ignored
// running 1 test
// test tests::ignored_test ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
//
// Doc-tests tmp-ignore
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

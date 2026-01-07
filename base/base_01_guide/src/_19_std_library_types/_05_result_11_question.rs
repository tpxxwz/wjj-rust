// ?
// 使用 match 链式处理结果可能会变得相当混乱；幸运的是，我们可以使用 ? 运算符来让代码变得整洁。
// ? 运算符用在返回 Result 的表达式末尾，等效于一个 match 表达式。在这个表达式中，Err(err) 分支会展开为提前返回的 return Err(From::from(err))，而 Ok(ok) 分支则展开为 ok 表达式。

mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // 中间函数
    fn op_(x: f64, y: f64) -> MathResult {
        // 如果 `div` "失败"，则会 `return` `DivisionByZero`
        let ratio = div(x, y)?;

        // 如果 `ln` "失败"，则会 `return` `NonPositiveLogarithm`
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!(
                "{}",
                match why {
                    MathError::NonPositiveLogarithm => "非正数的对数",
                    MathError::DivisionByZero => "除以零",
                    MathError::NegativeSquareRoot => "负数的平方根",
                }
            ),
            Ok(value) => println!("{}", value),
        }
    }
}

#[test]
fn main() {
    checked::op(1.0, 10.0);
}

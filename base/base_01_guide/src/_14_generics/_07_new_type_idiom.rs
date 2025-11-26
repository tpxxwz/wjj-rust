// 新类型惯用法
// newtype 模式在编译时保证了程序接收到正确类型的值。
//
// 例如，一个检查年龄（以年为单位）的年龄验证函数，必须接收 Years 类型的值。


struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    /// 截断不足一年的部分
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool {
    age.0 >= 18
}

#[wjj_lib::gen_test]
fn main() {
    let age = Years(25);
    let age_days = age.to_days();
    println!("是否成年？{}", is_adult(&age));
    println!("是否成年？{}", is_adult(&age_days.to_years()));
    // println!("是否成年？{}", is_adult(&age_days));

    // 取消最后一个 print 语句的注释，你会发现所提供的类型必须是 Years。
    //
    // 要获取 newtype 的基本类型值，你可以使用元组语法或解构语法，如下所示：

    let years = Years(42);
    let years_as_primitive_1: i64 = years.0; // 元组语法
    let Years(years_as_primitive_2) = years; // 解构语法
}



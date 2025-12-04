#[test]
fn array_test() {
    // 1. 创建一个固定长度数组
    let mut arr: [i32; 5] = [10, 20, 30, 40, 50];
    println!("原始数组: {:?}", arr);

    // 2. 访问数组元素
    println!("第一个元素: {}", arr[0]);
    println!("最后一个元素: {}", arr[arr.len() - 1]);

    // 3. 修改数组元素
    arr[2] = 99;
    println!("修改后的数组: {:?}", arr);

    // 4. 创建不可变切片（整个数组）
    let slice_all: &[i32] = &arr[..];
    println!("不可变切片（整个数组）: {:?}", slice_all);

    // 5. 创建部分切片
    let slice_part: &[i32] = &arr[1..4];
    println!("不可变切片（索引1到3）: {:?}", slice_part);

    // 6. 动态范围切片
    let n = 3;
    println!("动态切片 [..n]: {:?}", &arr[..n]);

    // 7. 遍历切片
    println!("遍历切片 [..n]:");
    for val in &arr[..n] {
        println!("值: {}", val);
    }

    // 8. 创建可变切片并修改元素
    {
        let slice_mut: &mut [i32] = &mut arr[1..4];
        slice_mut[0] = 777; // 修改 arr[1]
        println!("可变切片修改后: {:?}", slice_mut);
    }

    // 9. 验证数组长度未改变，但元素值已修改
    println!("最终数组: {:?}", arr);
    println!("数组长度: {}", arr.len());
}

#[test]
fn micro_vec_test() {
    // 1. 列举多个元素
    let v1 = vec![1, 2, 3, 4];
    println!("v1: {:?}", v1);

    // 2. 重复某个元素 N 次
    let buf = vec![0; 1024];
    println!("buf 长度: {}", buf.len());

    let repeated = vec!["rust"; 3];
    println!("repeated: {:?}", repeated);

    // 3. 显式指定类型
    let v2: Vec<i32> = vec![10, 20, 30];
    println!("v2: {:?}", v2);

    // 4. 嵌套使用（二维 Vec）
    let matrix = vec![vec![0; 3]; 3];
    println!("matrix: {:?}", matrix);

    // 5. 与推导结合（String 类型）
    let mixed = vec![String::from("a"), String::from("b")];
    // let mixed = vec![String::from("a"), "b".into()];
    println!("mixed: {:?}", mixed);
}

#[test]
fn vec_test() {
    // 1. 创建 Vec
    let empty: Vec<i32> = Vec::new();
    println!("空 Vec: {:?}", empty);

    let from_macro = vec![1, 2, 3];
    println!("使用 vec! 宏创建: {:?}", from_macro);

    let repeated = vec![0; 5];
    println!("重复元素: {:?}", repeated);

    // 2. 从数组或切片创建
    let from_array = Vec::from([10, 20, 30]);
    println!("从数组创建: {:?}", from_array);

    let from_slice = [1, 2, 3, 4].to_vec();
    println!("从切片创建: {:?}", from_slice);

    // 3. 添加元素
    let mut v = Vec::new();
    v.push(100);
    v.push(200);
    println!("push 后: {:?}", v);

    // 4. 删除元素
    v.pop(); // 删除最后一个
    println!("pop 后: {:?}", v);

    // 5. 访问元素
    println!("索引访问 v[0]: {}", v[0]);
    println!("get 安全访问: {:?}", v.get(0));

    // 6. 遍历
    for x in &v {
        println!("遍历元素: {}", x);
    }

    // 7. 修改元素
    if let Some(first) = v.get_mut(0) {
        *first = 999;
    }
    println!("修改后: {:?}", v);

    // 8. 插入和移除指定位置
    v.insert(0, 111);
    println!("insert 后: {:?}", v);

    v.remove(0);
    println!("remove 后: {:?}", v);

    // 9. 扩容和容量
    println!("长度: {}, 容量: {}", v.len(), v.capacity());
    v.reserve(10); // 预留容量
    println!("预留后容量: {}", v.capacity());

    // 10. 清空
    v.clear();
    println!("clear 后: {:?}", v);

    // 11. 排序
    let mut nums = vec![3, 1, 4, 2];
    nums.sort();
    println!("排序后: {:?}", nums);

    // 12. 反转
    nums.reverse();
    println!("反转后: {:?}", nums);

    // 13. 切片操作
    let slice = &nums[1..3];
    println!("切片: {:?}", slice);

    // 14. 合并 Vec
    let mut a = vec![1, 2];
    let b = vec![3, 4];
    a.extend(b);
    println!("合并后: {:?}", a);

    // 15. 转换为迭代器
    let sum: i32 = a.iter().sum();
    println!("元素求和: {}", sum);

    // 16. 使用 collect 从迭代器生成 Vec
    let collected: Vec<i32> = (0..5).collect();
    println!("collect 生成: {:?}", collected);

    // 17. 去重（需要排序）
    let mut dup = vec![1, 2, 2, 3, 3];
    dup.sort();
    dup.dedup();
    println!("去重后: {:?}", dup);
}

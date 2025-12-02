#[wjj_lib::gen_test]
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

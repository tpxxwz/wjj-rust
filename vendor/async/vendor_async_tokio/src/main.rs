#[tokio::main]
async fn main() {
    println!("hello");
}
// 等效于
// fn main() {
//     let mut rt = vendor_async_tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//         println!("hello");
//     })
// }
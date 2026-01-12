#[tokio::test]
async fn hello_world() -> mini_redis::Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = mini_redis::client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}

async fn say_world() {
    println!("world");
}

#[tokio::test]
async fn async_await() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This println! comes first
    println!("hello");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}

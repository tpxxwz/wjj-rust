use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool, Row};
use std::env;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

// export DATABASE_URL=postgres://root:wzzst310@localhost:5432/root
// sudo apt-get update
// sudo apt-get install -y pkg-config libssl-dev
// cargo install sqlx-cli --features postgres

// windows
// cargo install sqlx-cli --features postgres
// cargo sqlx --help
// cargo sqlx prepare -- --lib
// cargo sqlx prepare -- --lib --tests

// 0.8 0.9版本 cicd编译时用如下的
// export SQLX_OFFLINE=true
// cargo build --release

// sqlx migrate add create_users_table
// sqlx migrate run

async fn get_pool() -> PgPool {
    dotenv().ok();
    // from_filename(".env_prod").ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:wzzst310@localhost:5432/postgres".to_string());
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap()
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

#[tokio::test]
async fn test_create_table() {
    let pool = get_pool().await;
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            age INT NOT NULL
        );
        "#,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_add_index() {
    let pool = get_pool().await;
    pool.execute("CREATE INDEX IF NOT EXISTS idx_users_age ON users(age);")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_add_column() {
    let pool = get_pool().await;
    pool.execute("ALTER TABLE users ADD COLUMN IF NOT EXISTS email TEXT;")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_crud_operations() {
    let pool = get_pool().await;

    // Create
    sqlx::query!("INSERT INTO users (name, age) VALUES ($1, $2)", "Alice", 30)
        .execute(&pool)
        .await
        .unwrap();

    // Read
    let row = sqlx::query!("SELECT id, name, age FROM users WHERE name = $1", "Alice")
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("Read: {} {}", row.name, row.age);

    // Update

    sqlx::query!("UPDATE users SET age = $1 WHERE id = $2", 35, row.id as i32)
        .execute(&pool)
        .await
        .unwrap();

    // Delete
    sqlx::query!("DELETE FROM users WHERE id = $1", row.id as i32)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_crud_operations_query() {
    let pool = get_pool().await;

    // Create
    sqlx::query("INSERT INTO users (name, age) VALUES ('Alice', 30)")
        .execute(&pool)
        .await
        .unwrap();

    // Read
    let row = sqlx::query("SELECT id, name, age FROM users WHERE name = 'Alice'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let id: i32 = row.get("id");
    let name: String = row.get("name");
    let age: i32 = row.get("age");
    println!("Read: {} {}", name, age);

    // Update
    sqlx::query("UPDATE users SET age = $1 WHERE id = $2")
        .bind(35)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    // Delete
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_query_as_macro() -> Result<(), sqlx::Error> {
    let pool = get_pool().await;

    // 插入测试数据
    sqlx::query!("INSERT INTO users (name, age) VALUES ($1, $2)", "Alice", 30)
        .execute(&pool)
        .await?;

    // 使用 query_as! 宏查询并映射到 User 结构体
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, age FROM users WHERE name = $1",
        "Alice"
    )
    .fetch_one(&pool)
    .await?;

    // 验证结果
    assert_eq!(user.name, "Alice");
    assert_eq!(user.age, 30);
    assert_ne!(user.id, 0);

    Ok(())
}

#[tokio::test]
async fn test_transaction_rollback() -> Result<(), sqlx::Error> {
    let pool = get_pool().await;
    let mut tx = pool.begin().await?;

    sqlx::query("INSERT INTO users (name, age) VALUES ($1, $2)")
        .bind("Bob")
        .bind(40)
        .execute(&mut *tx)
        .await?;

    tx.rollback().await?;

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE name = 'Bob'")
        .fetch_one(&pool)
        .await?;
    assert_eq!(count, 0);

    Ok(())
}

#[tokio::test]
async fn test_transaction_multiple_sql() -> Result<(), sqlx::Error> {
    let pool = get_pool().await;
    let mut tx = pool.begin().await?;

    sqlx::query("INSERT INTO users (name, age) VALUES ($1, $2)")
        .bind("Charlie")
        .bind(28)
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO users (name, age) VALUES ($1, $2)")
        .bind("Dave")
        .bind(32)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE name IN ('Charlie','Dave')")
            .fetch_one(&pool)
            .await?;
    assert_eq!(count, 2);

    Ok(())
}

fn main() {}

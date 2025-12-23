use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};
use std::env;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

// export DATABASE_URL=postgres://root:wzzst310@localhost:5432/root
// sudo apt-get update
// sudo apt-get install -y pkg-config libssl-dev
// cargo install sqlx-cli --features postgres

async fn get_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://root:wzzst310@localhost:5432/root".to_string());
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap()
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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("配置错误: {0}")]
    ConfigError(String),
}

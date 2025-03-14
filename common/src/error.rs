#[derive(thiserror::Error, Debug)]
pub enum BrownFoxError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlite::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
}

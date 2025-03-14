#[derive(thiserror::Error, Debug)]
pub enum BrownFoxError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlite::Error),
}

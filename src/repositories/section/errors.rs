use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("not found id is: {0}")]
    NotFound(i32),
}

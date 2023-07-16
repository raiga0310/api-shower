pub mod section;
use thiserror::Error;

#[derive(Error, Debug)]
enum RepositoryError {
    #[error("Unexpected Error: [{0}]")]
    UnexpectedError(String),
    #[error("Not Found: [{0}]")]
    NotFound(String),
    #[error("Already Exists: [{0}]")]
    AlreadyExists(String),
}

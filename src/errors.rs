use thiserror::Error;

#[derive(Debug,Error,PartialEq)]
pub enum CacheError{
    #[error("key not found")]
    NotFound,
    #[error("invalid input:{0}")]
    InvalidInput(String)
}
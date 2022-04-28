use validation::ValidationError;

pub mod datetime;
pub mod post;
pub mod validation;

pub use post::Post;

pub(self) type Result<T> = std::result::Result<T, DomainError>;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("field validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("invalid boolean value: {0}")]
    ParseBool(#[from] std::str::ParseBoolError),
}

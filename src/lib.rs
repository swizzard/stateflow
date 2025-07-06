use thiserror::Error;

mod predicate;
pub mod prelude;
mod source;

#[derive(Error, Debug)]
pub enum SFError {
    #[error("io error")]
    IO(#[from] std::io::Error),
}

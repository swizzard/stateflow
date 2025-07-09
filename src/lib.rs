//! # Stateflow
//! An attempt at a modular, configurable decisioning system

#![warn(missing_docs)]
use thiserror::Error;

pub mod predicate;
pub mod prelude;
pub mod source;

/// Custom error type
#[derive(Error, Debug)]
pub enum SFError {
    /// Error caused by I/O
    #[error("io error")]
    IO(#[from] std::io::Error),
}

use thiserror::Error;

pub mod predicate;
pub mod source;

pub use predicate::{
    And, BinaryCombinable, Combinable, Not, OnDoW, Or, Predicate, TemporalPredicate,
};
pub use source::Source;

#[derive(Error, Debug)]
pub enum SFError {
    #[error("io error")]
    IO(#[from] std::io::Error),
}

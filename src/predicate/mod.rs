//! # [`Predicate`]s (and [`Combinable`]s)
//!
//! A [`Predicate`] maps [`Predicate::Data`] to a `bool`    
//! [`Combinable`] is an abstraction that allows for things like `(A ∧ B) ∨ ~C`

pub mod helpers;
pub mod traits;
pub use helpers::{And, Not, OnDoW, Or};
pub use traits::{BinaryCombinable, Combinable, Predicate};

pub mod temporal;
pub use temporal::TemporalPredicate;

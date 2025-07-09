//! # Sources of evaluable data
//!
//! It is expected that [`Source`] will be implemented multiple times with different [`Source::Data`] types    
//! It is also expected that [`Source::Data`] unifies with [`crate::predicate::Predicate::Data`]
//! elsewhere

use crate::SFError;
use std::future::{Future, ready};

/// A source of data to evaluate
pub trait Source {
    /// The data produced by this [`Source`]
    type Data: Send;

    /// Get [`Source::Data`]
    fn get_data(&self) -> impl Future<Output = Result<Self::Data, SFError>> + Send;
}

/// [`Source`] that always delivers the same content
#[derive(Debug)]
pub struct ConstSource<D: Send + Clone> {
    data: D,
}

impl<D> ConstSource<D>
where
    D: Send + Clone,
{
    /// Create a new [`ConstSource`]
    pub fn new(data: D) -> Self {
        Self { data }
    }
}

impl<D> Source for ConstSource<D>
where
    D: Send + Clone,
{
    type Data = D;

    fn get_data(&self) -> impl Future<Output = Result<Self::Data, SFError>> + Send {
        ready(Ok(self.data.clone()))
    }
}

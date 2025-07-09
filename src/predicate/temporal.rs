//! # Predicates that factor in time
use super::Predicate;
use chrono::{DateTime, Utc};

/// A kind of [`Predicate`] that also evaluates a [`DateTime`]
pub trait TemporalPredicate {
    /// Data evaluated
    type Data: Send;
    /// Evaluate data and given [`DateTime`]
    fn evaluate_with_datetime(&self, data: &Self::Data, date: DateTime<Utc>) -> bool;
    /// Evaluate data at current [`DateTime`]
    fn evaluate_now(&self, data: &Self::Data) -> bool {
        let now = Utc::now();
        self.evaluate_with_datetime(data, now)
    }
}

impl<P, D> Predicate for P
where
    P: TemporalPredicate<Data = D>,
    D: Send,
{
    type Data = D;
    fn evaluate(&self, data: &Self::Data) -> bool {
        self.evaluate_now(data)
    }
}

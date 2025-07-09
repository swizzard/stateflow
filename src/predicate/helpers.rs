//! # Pre-defined [`Predicate`]s and [`Combinable`]s

use super::temporal::TemporalPredicate;
use super::traits::{BinaryCombinable, Combinable, Predicate};
use chrono::{DateTime, Datelike, Utc};

/// Negate a [`Predicate`]
#[derive(Clone)]
pub struct Not<C, D>(C)
where
    D: Send,
    C: Combinable<Data = D>;

impl<C, D> Not<C, D>
where
    D: Send,
    C: Combinable<Data = D>,
{
    /// Create a new [`Not`]
    pub fn new(c: C) -> Self {
        Self(c)
    }
}

impl<C, D> Predicate for Not<C, D>
where
    D: Send,
    C: Combinable<Data = D>,
{
    type Data = D;
    fn evaluate(&self, data: &Self::Data) -> bool {
        !self.0.clone().finalize().evaluate(data)
    }
}

/// Logically `and` two [`Combinable`]s
#[derive(Clone)]
pub struct And<D, L, R>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    left: L,
    right: R,
}

impl<D, L, R> And<D, L, R>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    /// Create a new [`And`]
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<D, L, R> BinaryCombinable<D, L, R> for And<D, L, R>
where
    D: Send + Clone,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    fn combine(left: L, right: R) -> Self {
        Self::new(left, right)
    }
}

impl<D, L, R> Combinable for And<D, L, R>
where
    D: Send + std::clone::Clone,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    type Data = D;
    fn finalize(self) -> impl Predicate<Data = D> {
        Anded {
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

struct Anded<D, L, R>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    left: L,
    right: R,
}

impl<D, L, R> Predicate for Anded<D, L, R>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    type Data = D;
    fn evaluate(&self, data: &Self::Data) -> bool {
        self.left.clone().finalize().evaluate(data) && self.right.clone().finalize().evaluate(data)
    }
}

/// Logically `or` two [`Combinable`]s
#[derive(Clone)]
pub struct Or<D, L, R>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    left: L,
    right: R,
}

impl<D, L, R> Or<D, L, R>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    /// Create a new [`Or`]
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<D, L, R> BinaryCombinable<D, L, R> for Or<D, L, R>
where
    D: Send + Clone,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    fn combine(left: L, right: R) -> Self {
        Self::new(left, right)
    }
}

impl<D, L, R> Combinable for Or<D, L, R>
where
    D: Send + std::clone::Clone,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    type Data = D;
    fn finalize(self) -> impl Predicate<Data = D> {
        Ored {
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

struct Ored<D, L, R>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    left: L,
    right: R,
}

impl<D, L, R> Predicate for Ored<D, L, R>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    type Data = D;
    fn evaluate(&self, data: &Self::Data) -> bool {
        self.left.clone().finalize().evaluate(data) || self.right.clone().finalize().evaluate(data)
    }
}

/// [`TemporalPredicate`] that ignores `D` and evaluates the day of the week
#[derive(Clone)]
pub struct OnDoW<D> {
    on_day: chrono::Weekday,
    _data: std::marker::PhantomData<D>,
}

impl<D> OnDoW<D>
where
    D: Clone,
{
    fn new(on_day: chrono::Weekday) -> Self {
        Self {
            on_day,
            _data: std::marker::PhantomData,
        }
    }
    /// Create a new [`OnDoW`] that checks if it's Monday
    pub fn on_mon() -> Self {
        OnDoW::new(chrono::Weekday::Mon)
    }
    /// Create a new [`OnDoW`] that checks if it's Tuesday
    pub fn on_tues() -> Self {
        OnDoW::new(chrono::Weekday::Tue)
    }
    /// Create a new [`OnDoW`] that checks if it's Wednesday
    pub fn on_wed() -> Self {
        OnDoW::new(chrono::Weekday::Wed)
    }
    /// Create a new [`OnDoW`] that checks if it's Thursday
    pub fn on_thu() -> Self {
        OnDoW::new(chrono::Weekday::Thu)
    }
    /// Create a new [`OnDoW`] that checks if it's Friday
    pub fn on_fri() -> Self {
        OnDoW::new(chrono::Weekday::Fri)
    }
    /// Create a new [`OnDoW`] that checks if it's Saturday
    pub fn on_sat() -> Self {
        OnDoW::new(chrono::Weekday::Sat)
    }
    /// Create a new [`OnDoW`] that checks if it's Sunday
    pub fn on_sun() -> Self {
        OnDoW::new(chrono::Weekday::Sun)
    }
}

impl<D> TemporalPredicate for OnDoW<D>
where
    D: Send,
{
    type Data = D;

    fn evaluate_with_datetime(&self, _data: &Self::Data, date: DateTime<Utc>) -> bool {
        date.weekday() == self.on_day
    }
}

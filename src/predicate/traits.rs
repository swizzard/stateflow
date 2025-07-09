//! # Core predicate traits    

/// An evaluation of [`Predicate::Data`] to produce a `bool`
pub trait Predicate {
    /// Data evaluated
    type Data: Send;
    /// Evaluate the data
    fn evaluate(&self, data: &Self::Data) -> bool;
}

/// A wrapper trait abstracting over [`Predicate`]s and related higher-order types
pub trait Combinable: Clone {
    /// Data evaluated by the underlying [`Predicate`]
    type Data: Send;
    /// Produce the underlying [`Predicate`]
    fn finalize(self) -> impl Predicate<Data = Self::Data>;
}

/// Every [`Predicate`] is [`Combinable`]
impl<P, D> Combinable for P
where
    P: Predicate<Data = D> + Clone,
    D: Send,
{
    type Data = D;
    fn finalize(self) -> impl Predicate<Data = D> {
        self
    }
}

/// A trait representing operations over two [`Combinable`]s with the same [`Predicate::Data`] to
/// produce a new [`Combinable`]
pub trait BinaryCombinable<D, L, R>: Combinable<Data = D>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    /// Combine two [`Combinable`]s
    fn combine(left: L, right: R) -> Self;
}

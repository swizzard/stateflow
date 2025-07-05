pub trait Predicate {
    type Data: Send;
    fn evaluate(&self, data: &Self::Data) -> bool;
}

pub trait Combinable: Clone {
    type Data: Send;
    fn finalize(self) -> impl Predicate<Data = Self::Data>;
}

pub trait BinaryCombinable<L, R, D>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    fn combine(left: L, right: R) -> Self;
}

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

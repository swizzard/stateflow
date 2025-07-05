use super::traits::{BinaryCombinable, Combinable, Predicate};

pub struct Not<C, D>(C)
where
    D: Send,
    C: Combinable<Data = D>;

impl<C, D> Not<C, D>
where
    D: Send,
    C: Combinable<Data = D>,
{
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

#[derive(Clone)]
pub struct And<L, R, D>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    left: L,
    right: R,
}

impl<L, R, D> And<L, R, D>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R, D> BinaryCombinable<L, R, D> for And<L, R, D>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    fn combine(left: L, right: R) -> Self {
        Self::new(left, right)
    }
}

impl<L, R, D> Combinable for And<L, R, D>
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

struct Anded<L, R, D>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    left: L,
    right: R,
}

impl<L, R, D> Predicate for Anded<L, R, D>
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

#[derive(Clone)]
pub struct Or<L, R, D>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    left: L,
    right: R,
}

impl<L, R, D> Or<L, R, D>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R, D> BinaryCombinable<L, R, D> for Or<L, R, D>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    fn combine(left: L, right: R) -> Self {
        Self::new(left, right)
    }
}

impl<L, R, D> Combinable for Or<L, R, D>
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

struct Ored<L, R, D>
where
    D: Send,
    L: Combinable<Data = D>,
    R: Combinable<Data = D>,
{
    left: L,
    right: R,
}

impl<L, R, D> Predicate for Ored<L, R, D>
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

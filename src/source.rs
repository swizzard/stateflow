use crate::SFError;
use std::future::{Future, ready};

pub trait Source {
    type Data: Send;

    fn get_data(&self) -> impl Future<Output = Result<Self::Data, SFError>> + Send;
}

#[derive(Debug)]
pub struct ConstSource<D: Send + Clone> {
    data: D,
}

impl<D> ConstSource<D>
where
    D: Send + Clone,
{
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

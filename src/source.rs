use crate::SFError;
use std::future::Future;

pub trait Source {
    type Data: Send;

    fn get_data(&self) -> impl Future<Output = Result<Self::Data, SFError>> + Send;
}

use crate::IntoResponse;
use std::future::Future;

pub type Request = hyper::Request<hyper::Body>;

pub trait FromRequest: Sized {
    type ResultFuture: Future<Output = Result<Self, Self::Error>>;
    type Error: IntoResponse;

    fn from_request(request: &Request) -> Self::ResultFuture;
}

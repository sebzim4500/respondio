use std::future::Future;
use hyper::Body;
use futures::future::{ready, Ready};

pub type Response = hyper::Response<hyper::Body>;

pub trait IntoResponse {
    type ResultFuture: Future<Output = Response>;

    fn into_response(self) -> Self::ResultFuture;
}

impl IntoResponse for () {
    type ResultFuture = Ready<Response>;

    fn into_response(self) -> Self::ResultFuture {
        ready(Response::new(Body::empty()))
    }
}

impl IntoResponse for &'static str {
    type ResultFuture = Ready<Response>;

    fn into_response(self) -> Self::ResultFuture {
        ready(Response::new(Body::from(self)))
    }
}
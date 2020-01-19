use futures::future::{ready, Ready};
use hyper::{Body, StatusCode};
use std::future::Future;

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

impl IntoResponse for String {
    type ResultFuture = Ready<Response>;

    fn into_response(self) -> Self::ResultFuture {
        ready(Response::new(Body::from(self)))
    }
}

pub fn parse_failure(arg_name: &str) -> Response {
    let mut response = Response::new(Body::from(format!("Could not parse variable {}", arg_name)));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

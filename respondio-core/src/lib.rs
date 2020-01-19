pub use hyper::Method;
use std::future::Future;

pub mod request;
pub mod response;
pub mod routing;

use futures::future::{ready, Either};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Server, StatusCode};
pub use request::{FromRequest, Request};
pub use response::{IntoResponse, Response};
pub use routing::Route;
use routing::RouteTree;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Clone)]
pub struct RouteHandler {
    pub fn_name: String,
    pub method: Method,
    pub path: String,
    pub handler: fn(
        Request,
        Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, Infallible>> + Send>>,
}

inventory::collect!(RouteHandler);

impl RouteHandler {
    pub fn new(
        fn_name: String,
        method: Method,
        path: String,
        handler: fn(
            Request,
            Vec<String>,
        ) -> Pin<Box<dyn Future<Output = Result<Response, Infallible>> + Send>>,
    ) -> Self {
        RouteHandler {
            fn_name,
            method,
            path,
            handler,
        }
    }
}

struct RoutingTable {
    tree: RouteTree<RouteHandler>,
}

impl RoutingTable {
    fn process_request(
        &self,
        request: Request,
    ) -> impl Future<Output = Result<Response, Infallible>> + 'static {
        if let Some((handler, path_vars)) = self.tree.match_path(request.uri().path()) {
            Either::Left((handler.handler)(request, path_vars))
        } else {
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::NOT_FOUND;
            Either::Right(ready(Ok(response)))
        }
    }
}

pub async fn run_server(addr: &SocketAddr) {
    let mut tree = RouteTree::default();

    for handler in inventory::iter::<RouteHandler> {
        println!("Found handler with name {}", handler.fn_name);
        tree.add_route(Route::new(&handler.path), handler.clone());
    }

    let table = Arc::new(RoutingTable { tree });

    let server = Server::bind(addr).serve(make_service_fn(move |_| {
        let table_clone = table.clone();
        async {
            Ok::<_, Infallible>(service_fn(move |request| {
                table_clone.process_request(request)
            }))
        }
    }));
    server.await.expect("Running hyper server");
}

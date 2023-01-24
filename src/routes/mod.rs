mod handlers;

use handlers::{not_found, root, transpose};
use hyper::Method;
use hyper::{Body, Request, Response};
use std::convert::Infallible;

pub async fn routing(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (_req.method(), _req.uri().path()) {
        (&Method::POST, "/T") => transpose(_req).await,
        (&Method::GET, "/") => root().await,
        _ => not_found().await,
    }
}

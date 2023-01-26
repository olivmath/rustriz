use crate::domain::matrix::Matrix_3x3;
use hyper::{Body, Request, Response};
use hyper::{Method, StatusCode};
use serde_json::Error;
use std::convert::Infallible;

async fn req_to_matrix(req: Request<Body>) -> Result<Matrix_3x3, Error> {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    serde_json::from_str(&body_str)
}

pub async fn transpose(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let matrix: Matrix3x3 = match req_to_type(req).await {
        Ok(m) => m,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid Request Body".into())
                .unwrap())
        }
    };

    let res = match serde_json::to_string(&matrix.transpose()) {
        Ok(s) => s,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Internal Server Error: {:?}", e).into())
                .unwrap())
        }
    };
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(res.into())
        .unwrap())
}

pub async fn root() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("Rustriz say: Hello 🧮!".into())
        .unwrap())
}

pub async fn not_found() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("404 Not Found".into())
        .unwrap())
}

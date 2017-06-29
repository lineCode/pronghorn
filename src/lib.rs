extern crate hyper;
extern crate futures;

use futures::future;
use hyper::error::Error;

pub mod server;
pub mod router;

pub type Future = future::FutureResult<Response, Error>;
pub type Request = hyper::server::Request;
pub type Response = hyper::server::Response;
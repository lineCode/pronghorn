use std::sync::Arc;
use std::net::SocketAddr;
use hyper::server::{Http, Request, Response, Service};
use hyper::error::Error;
use futures;

use router::Router;

#[derive(Debug)]
pub struct Server {
    pub router: Router
}

impl Server {
    pub fn new() -> Server {
        Server {
            router: Router::new()
        }
    }

    pub fn run(self, addr: &SocketAddr) {
        println!("Starting server on {}...", addr);
        let s = Arc::new(self);
        let http = Http::new().bind(&addr, move || Ok(s.clone())).unwrap();
        http.run().unwrap();
    }
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, _req: Request) -> Self::Future {
        let mut res = Response::new();
        res.set_body("Pronghorn says \"Hello, World!\"");
        futures::future::ok(res)
    }
}
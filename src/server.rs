use std::net::SocketAddr;
use hyper::server::{Http, Request, Response, Service};
use hyper::error::Error;
use futures;

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn run(&self, addr: &SocketAddr) {
        println!("Starting server on {}...", addr);
        let http = Http::new().bind(&addr, || Ok(Server)).unwrap();
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
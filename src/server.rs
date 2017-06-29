use std::sync::Arc;
use std::net::SocketAddr;
use hyper::server::{Http, Service};
use hyper::error::Error;
use futures;
use hyper::StatusCode;

use router::Router;

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
    type Request = ::Request;
    type Response = ::Response;
    type Error = Error;
    type Future = ::Future;

    fn call(&self, req: ::Request) -> ::Future {
        for h in self.router.routes.iter() {
            if h.method == *req.method() && h.path == *req.path() {
                return (h.handler)(&req);
            }
        }
        let mut notfound = ::Response::new();
        notfound.set_status(StatusCode::NotFound);
        return futures::future::ok(notfound);
    }
}
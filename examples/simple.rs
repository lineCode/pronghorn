extern crate pronghorn;
extern crate futures;

use pronghorn::server::{Server};
use pronghorn::{Response, Request, Future};

fn foo(_req: &Request) -> Future {
    let mut res = Response::new();
    res.set_body("Foo");
    futures::future::ok(res)
}

fn bar(_req: &Request) -> Future {
    let mut res = Response::new();
    res.set_body("Bar");
    futures::future::ok(res)
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let mut server = Server::new();
    server.router.get("/", foo);
    server.router.get("/bar", bar);
    server.run(&addr);
}
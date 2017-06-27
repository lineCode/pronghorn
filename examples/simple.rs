extern crate pronghorn;

use pronghorn::server::Server;

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let mut server = Server::new();
    server.router.get("/");
    server.router.get("/blah");
    server.run(&addr);
}
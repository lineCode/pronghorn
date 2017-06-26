extern crate pronghorn;

use pronghorn::server::Server;

fn main() {
    let server = Server::new();
    let addr = "127.0.0.1:3000".parse().unwrap();
    server.run(&addr);
}
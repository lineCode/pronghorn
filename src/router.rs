use hyper::Method;
use hyper::server::Request;

pub struct Router {
    pub routes: Vec<Route>
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: Vec::new()
        }
    }

    pub fn get(&mut self, path: &str, handler: fn(&Request) -> ::Future) {
        self.routes.push(Route::new(Method::Get, path, handler));
    }
}

pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler: fn(&Request) -> ::Future
}

impl Route {
    pub fn new(method: Method, path: &str, handler: fn(&Request) -> ::Future) -> Route {
        Route {
            method: method,
            path: String::from(path),
            handler: handler
        }
    }
}
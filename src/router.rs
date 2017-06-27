#[derive(Debug)]
pub struct Router {
    pub routes: Vec<Route>
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: Vec::new()
        }
    }

    pub fn get(&mut self, path: &str) {
        self.routes.push(Route::new("get", path));
    }
}

#[derive(Debug)]
pub struct Route {
    pub method: String,
    pub path: String
}

impl Route {
    pub fn new(method: &str, path: &str) -> Route {
        Route {
            method: String::from(method),
            path: String::from(path)
        }
    }
}
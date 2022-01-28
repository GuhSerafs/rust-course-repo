use std::{collections::HashMap};

#[allow(dead_code)]
pub struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[allow(dead_code)]
pub struct Response {
    code: u32, 
    headers: HashMap<String, String>, 
    body: Vec<u8>,
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

pub struct BasicRouter {
    routes: HashMap<String, BoxedCallback>
}

impl BasicRouter {
    /// Create a Empty Router
    pub fn new() -> Self {
        BasicRouter { routes: HashMap::new() }
    }

    /// Add a route to the router
    pub fn add_route(&mut self, url: &str, callback: BoxedCallback) {
        self.routes.insert(url.to_string(), callback);
    }

    pub fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => Response {
                code: 404,
                headers: HashMap::new(),
                body: "<h1>Error 404: Not Found".as_bytes().to_vec(),                
            }, 
            Some(callback) => callback(request),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{BasicRouter, Request, Response};
    use std::collections::HashMap;

    #[test]
    fn unit_router () {
        let mut router = BasicRouter::new();
        router.add_route("/", Box::new(|_| Response {
            code: 200, 
            headers: HashMap::new(),
            body: "<title>GCD Calc </title>...".as_bytes().to_vec(),
        }));
        assert_eq!(router.routes.keys().into_iter().next(), Some(&"/".to_string()));
    }

    #[test]
    fn multiple_router () {
        let mut router = BasicRouter::new();
        router.add_route("/", Box::new(|_| Response {
            code: 200, 
            headers: HashMap::new(),
            body: "<title>GCD Calc </title>...".as_bytes().to_vec(),
        }));

        router.add_route("/gcd", Box::new(|req: &Request| Response {
            code: 204, 
            headers: req.headers.clone(),
            body: req.body.clone(),
        }));
        assert!(router.routes.contains_key("/"));
        assert!(router.routes.contains_key("/gcd"));
    }
}
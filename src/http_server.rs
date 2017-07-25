extern crate hyper;
extern crate futures;

use std::thread;

use self::hyper::header::ContentLength;
use self::hyper::server::{Http, Request, Response, Service};
use self::hyper::{StatusCode, Method};

#[derive(Clone)]
struct Server {
    html: String
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let (status, content) = match req.method() {
            &Method::Get => {
                match req.uri().as_ref() {
                    "/" => (StatusCode::Ok, self.html.clone()),
                    _ => (StatusCode::NotFound, "404".to_string()),
                }
            }
            _ => (StatusCode::MethodNotAllowed, "502".to_string()),
        };

        futures::future::ok(
            Response::new()
                .with_header(ContentLength(content.len() as u64))
                .with_status(status)
                .with_body(content),
        )
    }
}

use ::std::net::SocketAddr;

pub(crate) fn start(addr: SocketAddr, html: String) {
    thread::spawn(move || {
        let server = Server { html };
        let server = Http::new().bind(&addr, move || Ok(server.clone())).unwrap();
        println!("Started HTTP server on {}", server.local_addr().unwrap());
        server.run().unwrap();
    });
}

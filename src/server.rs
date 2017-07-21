extern crate hyper;
extern crate futures;

use std::thread;

use self::hyper::header::ContentLength;
use self::hyper::server::{Http, Request, Response, Service};
use self::hyper::{StatusCode, Method};

const HEADER: &'static str = include_str!("static/index.html");
const SCRIPT: &'static str = include_str!("static/script.js");
lazy_static! {
    static ref HTML: String = format!("{}<script>{}</script>", HEADER, SCRIPT);
}

struct Server;

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let (status, content) = match req.method() {
            &Method::Get => {
                match req.uri().as_ref() {
                    "/" => (StatusCode::Ok, HTML.as_bytes()),
                    _ => (StatusCode::NotFound, b"404" as &[u8]),
                }
            }
            _ => (StatusCode::MethodNotAllowed, b"502" as &[u8]),
        };

        futures::future::ok(
            Response::new()
                .with_header(ContentLength(content.len() as u64))
                .with_status(status)
                .with_body(content),
        )
    }
}

pub(crate) fn start(addr: ::std::net::SocketAddr) {
    thread::spawn(move || {
        let server = Http::new().bind(&addr, || Ok(Server)).unwrap();
        println!("Started HTTP server on {}", server.local_addr().unwrap());
        server.run().unwrap();
    });
}

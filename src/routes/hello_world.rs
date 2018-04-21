
extern crate hyper;
extern crate futures;

use routes::hello_world::futures::future::Future;
use routes::hello_world::hyper::header::ContentLength;
use routes::hello_world::hyper::server::{Request, Response, Service};


pub struct HelloWorld;

const PHRASE: &'static str = "Hello, World!";

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE)
        ))
    }
}

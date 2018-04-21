extern crate hyper;
extern crate futures;
//extern crate tokio_core;

use routes::echo::futures::{Future, Stream};
use routes::echo::hyper::server::{Request, Response, Service};
use routes::echo::hyper::{Body, Chunk};
use routes::echo::hyper::{Method, StatusCode};

pub struct Echo;

impl Service for Echo {
    type Request  = Request;
    type Response = Response<Box<Stream<Item=Chunk, Error=Self::Error>>>;
    type Error    = hyper::Error;
    type Future   = Box<Future<Item=Self::Response, Error=Self::Error>>; 

    fn call(&self, req: Request) -> Self::Future {
        
        let mut response = Response::new();
        
         match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let body: Box<Stream<Item=_, Error=_>> = Box::new(Body::from("Try POSTing to /echo!"));
                response.set_body(body);
            },
            (&Method::Post, "/echo") => {
                let mapping = req.body().map(to_uppercase as fn(Chunk) -> Chunk);
                let body: Box<Stream<Item=_, Error=_>> = Box::new(mapping);
                response.set_body(body);
            },
             _ => {
                response.set_status(StatusCode::NotFound);
            }            
        };
        Box::new(futures::future::ok(response))
    }    
}

fn to_uppercase(chunk: Chunk) -> Chunk {
    let uppered = chunk.iter()
            .map(|byte| byte.to_ascii_uppercase())
            .collect::<Vec<u8>>();
    Chunk::from(uppered)
}

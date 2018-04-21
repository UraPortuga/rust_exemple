extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;

use routes::file_response::futures::{Future};
use routes::file_response::hyper::{Get, Request, StatusCode, Body, Chunk, Uri};
use routes::file_response::hyper::header::{ContentLength, ContentType};
use routes::file_response::hyper::server::{Http, Request as OtherRequest, Response, Service};
use routes::file_response::futures::sync::{mpsc, oneshot};
use routes::file_response::hyper::error::Error;

use std::fs::File;
use std::io::{self, copy, Read};
use std::thread;

static NOTFOUND: &[u8] = b"Not Found";
static Path_File: &str = "C://Users/uracir.santos/projects/hyper_test/src/files/get.credit.card.bill.id.response.json";

fn simple_file_send(f: &str) -> Box<Future<Item = Response, Error = hyper::Error>> {
    let filename = f.to_string(); 
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                tx.send(Response::new()
                        .with_status(StatusCode::NotFound)
                        .with_header(ContentLength(NOTFOUND.len() as u64))
                        .with_body(NOTFOUND))
                    .expect("Send error on open");
                return;
            },
        };
        let mut buf: Vec<u8> = Vec::new();
        match copy(&mut file, &mut buf) {
            Ok(_) => {
                let res = Response::new()
                    .with_header(ContentType::json())                                        
                    .with_header(ContentLength(buf.len() as u64))
                    .with_body(buf);
                tx.send(res).expect("Send error on successful file read");
            },
            Err(_) => {
                tx.send(Response::new().with_status(StatusCode::InternalServerError)).
                    expect("Send error on error reading file");
            },
        };
    });

    Box::new(rx.map_err(|e| Error::from(io::Error::new(io::ErrorKind::Other, e))))
}

pub struct ResponseExamples;

impl Service for ResponseExamples {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Get, "/")=> {
                 simple_file_send(Path_File)                  
            },                
            _ => {
                 Box::new(futures::future::ok(Response::new()
                                    .with_status(StatusCode::NotFound)))
            }
        }
    }
}


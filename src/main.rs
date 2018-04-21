extern crate hyper;
extern crate hyper_test;

use hyper::server::Http;
use hyper_test::routes::hello_world::HelloWorld;
use hyper_test::routes::echo;
use hyper_test::routes::file_response::ResponseExamples;

fn main() {

    let addr = "127.0.0.1:1337".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(ResponseExamples)).unwrap();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap();

}
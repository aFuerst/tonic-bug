use tonic::{transport::Server, Request, Response, Status};

use pinger_lib::greeter_server::{Greeter, GreeterServer};
use pinger_lib::{HelloReply, HelloRequest};

#[derive(Default)]
pub struct MyGreeter {
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        std::thread::sleep(std::time::Duration::from_secs(10));
        let reply = HelloReply {
            message: format!("Hello from {}!", "dasd"),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();
    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
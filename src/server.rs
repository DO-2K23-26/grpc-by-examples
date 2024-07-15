use std::{borrow::Borrow, time::Duration};

use asynchello::{
    async_hello_server::{AsyncHello, AsyncHelloServer},
    HelloReply, HelloRequest,
};
use hello::{
    say_server::{Say, SayServer},
    SayRequest, SayResponse,
};
use tokio::sync::{mpsc, watch};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello");
}

pub mod asynchello {
    tonic::include_proto!("asynchello");
}

#[derive(Debug)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        println!("Got a request: {:?}", request);
        let reply = SayResponse {
            message: format!("Hello, {}!", request.get_ref().message),
        };
        Ok(Response::new(reply))
    }
}

#[derive(Debug)]
pub struct MyAsyncHello {}

#[tonic::async_trait]
impl AsyncHello for MyAsyncHello {
    type SayHelloStream = ReceiverStream<Result<HelloReply, Status>>;

    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::SayHelloStream>, Status> {
        let names = request.into_inner().name;
        let (tx, rx) = mpsc::channel(names.len());

        tokio::spawn(async move {
            for name in names {
                tx.send(Ok(HelloReply {
                    message: format!("Hello, {}!", name),
                }))
                .await
                .unwrap();
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let say = MySay {};
    let asynchello = MyAsyncHello {};
    Server::builder()
        .add_service(SayServer::new(say))
        .add_service(AsyncHelloServer::new(asynchello))
        .serve(addr)
        .await?;
    Ok(())
}

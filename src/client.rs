use std::borrow::Borrow;

use asynchello::{
    async_hello_client::AsyncHelloClient, async_hello_server::AsyncHello, HelloRequest,
};
use hello::{say_client::SayClient, SayRequest};
use tonic::{transport::Channel, Request};

pub mod hello {
    tonic::include_proto!("hello");
}

pub mod asynchello {
    tonic::include_proto!("asynchello");
}

#[derive(Debug)]
enum CustomError {
    ConnectionError(tonic::transport::Error),
    RequestError(tonic::Status),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://0.0.0.0:50051")
        .connect()
        .await
        .map_err(|err| {
            println!("Error: {:?}", err);
            err
        })?;

    // send_request(channel.clone()).await?;
    send_async_hello(channel.clone()).await?;
    Ok(())
}

async fn send_request(channel: Channel) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SayClient::new(channel);
    let var_name = SayRequest {
        message: "Tonic".into(),
    };
    let request = Request::new(var_name);

    let response = client.send(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}

async fn send_async_hello(channel: Channel) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AsyncHelloClient::new(channel);
    let request = Request::new(HelloRequest {
        name: vec!["Tonic".into(), "Djul".into(), "Zlatan".into()],
    });

    let mut stream = client.say_hello(request).await?.into_inner();
    while let Some(response) = stream.message().await? {
        println!("RESPONSE={:?}", response);
    }
    println!("Async hello done");
    Ok(())
}

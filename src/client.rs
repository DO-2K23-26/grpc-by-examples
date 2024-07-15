use hello::{say_client::SayClient, SayRequest};
use tonic::{transport::Channel, Request};

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = SayClient::new(channel);
    let var_name = SayRequest {
        message: "Tonic".into(),
    };
    let request = Request::new(var_name);

    let response = client.send(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}

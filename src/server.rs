use hello::{
    say_server::{Say, SayServer},
    SayRequest, SayResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello");
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let say = MySay {};
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}

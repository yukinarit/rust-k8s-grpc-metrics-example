use tonic::{transport::Server, Request, Response, Status};

use foo::foo_server::{Foo, FooServer};
use foo::{EchoReply, EchoRequest};

pub mod foo {
    tonic::include_proto!("foo"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct FooHandler {}

#[tonic::async_trait]
impl Foo for FooHandler {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = foo::EchoReply {
            message: request.into_inner().name.clone(),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:20000".parse()?;
    let foo = FooHandler::default();

    Server::builder()
        .add_service(FooServer::new(foo))
        .serve(addr)
        .await?;

    Ok(())
}

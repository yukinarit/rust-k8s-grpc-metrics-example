use metrics::{histogram, increment_counter, increment_gauge};
use std::time::Instant;
use tonic::{transport::Server, Request, Response, Status};

use foo::foo_server::{Foo, FooServer};
use foo::{EchoReply, EchoRequest};

pub mod foo {
    tonic::include_proto!("foo");
}

#[derive(Debug, serde::Deserialize)]
struct Config {
    host: String,
    port: u16,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Config {
            host: "0.0.0.0".into(),
            port: 20000,
        }
    }
}

impl Config {
    fn load() -> Config {
        let mut cfg = config::Config::default();
        cfg.merge(config::Environment::with_prefix("server"))
            .unwrap();
        cfg.try_into().unwrap_or_else(|_| Default::default())
    }
}

#[derive(Debug, Default)]
pub struct FooHandler {}

#[tonic::async_trait]
impl Foo for FooHandler {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoReply>, Status> {
        let now = Instant::now();

        println!("Got a request: {:?}", request);

        let reply = foo::EchoReply {
            message: request.into_inner().name.clone(),
        };

        histogram!("foo_histgram", now.elapsed(), "api" => "echo", "result" => "ok");

        increment_counter!("foo_counter");

        increment_gauge!("foo_gauge", 10.0);

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::load();
    println!("{:?}", cfg);

    let addr = format!("{}:{}", cfg.host, cfg.port).parse()?;
    let foo = FooHandler::default();

    metrics_exporter_prometheus::PrometheusBuilder::new().install()?;

    Server::builder()
        .add_service(FooServer::new(foo))
        .serve(addr)
        .await?;

    Ok(())
}

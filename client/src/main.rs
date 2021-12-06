use foo::foo_client::FooClient;
use foo::EchoRequest;

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
            host: "localhost".into(),
            port: 20000,
        }
    }
}

impl Config {
    fn load() -> Config {
        let mut cfg = config::Config::default();
        cfg.merge(config::Environment::with_prefix("client"))
            .unwrap();
        cfg.try_into().unwrap_or_else(|_| Default::default())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::load();
    println!("{:?}", cfg);
    let mut client = FooClient::connect(format!("http://{}:{}", cfg.host, cfg.port)).await?;

    let request = tonic::Request::new(EchoRequest {
        name: "Tonic".into(),
    });

    let response = client.echo(request).await?;

    println!("Response={:?}", response);

    Ok(())
}

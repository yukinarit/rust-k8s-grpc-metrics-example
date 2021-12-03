use foo::foo_client::FooClient;
use foo::EchoRequest;

pub mod foo {
    tonic::include_proto!("foo");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = FooClient::connect("http://localhost:20000").await?;

    let request = tonic::Request::new(EchoRequest {
        name: "Tonic".into(),
    });

    let response = client.echo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

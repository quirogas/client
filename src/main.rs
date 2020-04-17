use hyper::{Body, Client, Request };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = Request::get("http://httpbin.org").body(Body::from(""))?;

    let response = client.request(request).await?;
    println!("{:?}", response);

    Ok(())
}

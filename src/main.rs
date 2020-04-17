use hyper::{Body, Client, Request, Response  };
use hyper_tls::HttpsConnector;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::get("https://httpbin.org").body(Body::from(""))?;

    let response = send(request).await?;

    println!("after send: {:?}", response);

    Ok(())
}

async fn send(request: Request<Body>) -> Result<Response<Body>,Box<dyn Error>> {
 let client = Client::builder().build::<>(HttpsConnector::new());
 let response = client.request(request).await?;
    
 match  response.status() {
     hyper::StatusCode::OK => Ok(response),
     _ => Err("there was an error with the request".into())

 }
}

use reqwest::{Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


let client = Client::new();
let response_get = client.get("https://httpbin.org/get").send();
let response_post = client.post("http://httpbin.org/post").send();

let response_get_auth = client.get("https://httpbin.org/basic-auth/root/toor").basic_auth("root", Some("toor")).send();

    println!(" GET: {:?}\n\nPOST: {:?}\n\nAUTH: {:?}",response_get.await, response_post.await, response_get_auth.await);

    Ok(())
}

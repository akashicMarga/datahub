use std::io::stdin;

use text::{text_client::TextClient, TextRequest};

pub mod text {
    tonic::include_proto!("text");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TextClient::connect("http://[::1]:8080").await?;
    loop {
        let mut query = String::new();
        println!("Please enter a query: ");
        stdin().read_line(&mut query).unwrap();
        let request = tonic::Request::new(TextRequest { txt: query });
        let response = client.txt(request).await?;
        println!("Got: '{}' from service", response.into_inner().embedding);
    }
    Ok(())
}

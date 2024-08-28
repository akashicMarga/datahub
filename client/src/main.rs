use std::io::stdin;

use text::{text_client::TextClient, HealthCheckRequest, HealthCheckResponse, TextRequest};

pub mod text {
    tonic::include_proto!("text");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TextClient::connect("http://[::1]:8080").await?;

    // loop to genrate text embdeggings from server
    // loop {
    //     let mut query = String::new();
    //     println!("Please enter a query: ");
    //     stdin().read_line(&mut query).unwrap();
    //     let request = tonic::Request::new(TextRequest { txt: query });
    //     let response = client.txt(request).await?;
    //     println!("Got: '{}' from service", response.into_inner().embedding);
    // }

    //check health api
    let request = tonic::Request::new(HealthCheckRequest {
        duration_seconds: 30, // Run for 30 seconds
        interval_seconds: 5,  // Send updates every 5 seconds
    });

    let mut stream = client.health_check(request).await?.into_inner();

    while let Some(response) = stream.message().await? {
        println!("Health check update: {:?}", response);
    }
    Ok(())
}

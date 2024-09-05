use std::io::stdin;

use text::{
    text_client::TextClient, HealthCheckRequest, HealthCheckResponse, SearchRequest, TextRequest,
};

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
    // let request = tonic::Request::new(HealthCheckRequest {
    //     duration_seconds: 30, // Run for 30 seconds
    //     interval_seconds: 5,  // Send updates every 5 seconds
    // });

    // let mut stream = client.health_check(request).await?.into_inner();

    // while let Some(response) = stream.message().await? {
    //     println!("Health check update: {:?}", response);
    // }

    //search similar
    // Prepare the search request
    let request = tonic::Request::new(SearchRequest {
        query: "thanks".to_string(),
        limit: 5, // Number of results you want
    });

    // Send the request and get the response
    let response = client.search_similar(request).await?;

    // Process the response
    let search_results = response.into_inner().results;

    println!("Search Results:");
    for (index, result) in search_results.iter().enumerate() {
        println!(
            "{}. Text: {}, Score: {}",
            index + 1,
            result.text,
            result.score
        );
    }

    Ok(())
}

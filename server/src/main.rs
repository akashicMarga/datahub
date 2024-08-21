use text::{
    text_server::{Text, TextServer},
    TextRequest, TextResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod text {
    tonic::include_proto!("text");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();
    let text_service = TextService::default();

    Server::builder()
        .add_service(TextServer::new(text_service))
        .serve(address)
        .await?;
    Ok(())
}

#[derive(Debug, Default)]
pub struct TextService {}

#[tonic::async_trait]
impl Text for TextService {
    async fn txt(&self, request: Request<TextRequest>) -> Result<Response<TextResponse>, Status> {
        let r = request.into_inner();
        Ok(Response::new(text::TextResponse {
            confirmation: { format!("{}", r.txt) },
        }))
    }
}

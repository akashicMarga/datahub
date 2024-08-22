mod embeddings;
mod utils;

use embeddings::EmbeddingModel;
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
    let text_service = TextService::load_service();

    Server::builder()
        .add_service(TextServer::new(text_service))
        .serve(address)
        .await?;
    Ok(())
}

pub struct TextService {
    emb_model: embeddings::EmbeddingModel,
}

#[tonic::async_trait]
impl Text for TextService {
    async fn txt(&self, request: Request<TextRequest>) -> Result<Response<TextResponse>, Status> {
        let r = request.into_inner();
        let embeddings: Vec<f32> = self
            .emb_model
            .get_embeddings(&r.txt)
            .unwrap()
            .reshape((384,))
            .unwrap()
            .to_vec1()
            .unwrap();
        let embeddings = embeddings
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        Ok(Response::new(text::TextResponse {
            embedding: { embeddings },
        }))
    }
}

impl TextService {
    pub fn load_service() -> Self {
        let model = EmbeddingModel::load_model();
        Self { emb_model: model }
    }
}

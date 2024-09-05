use qdrant_client::qdrant::ScoredPoint;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct, ScalarQuantizationBuilder, SearchParamsBuilder,
    SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
};
use qdrant_client::{Payload, Qdrant};

pub struct QdrantService {
    client: Qdrant,
}

impl QdrantService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = Qdrant::from_url("http://localhost:6334").build()?;

        Ok(Self { client })
    }

    pub async fn init_collection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let collections = self.client.list_collections().await?;
        if !collections
            .collections
            .iter()
            .any(|c| c.name == "embeddings")
        {
            self.client
                .create_collection(
                    CreateCollectionBuilder::new("embeddings")
                        .vectors_config(VectorParamsBuilder::new(384, Distance::Cosine))
                        .quantization_config(ScalarQuantizationBuilder::default()),
                )
                .await?;
        }
        Ok(())
    }

    pub async fn store_embedding(
        &self,
        id: String,
        embedding: Vec<f32>,
        payload: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let collection_name = "embeddings";
        let payload: Payload = serde_json::json!(
            {
                 "text": payload
            }
        )
        .try_into()
        .unwrap();

        let points = vec![PointStruct::new(id, embedding, payload)];

        self.client
            .upsert_points(UpsertPointsBuilder::new(collection_name, points))
            .await?;
        Ok(())
    }

    pub async fn search_similar(
        &self,
        query_vector: Vec<f32>,
        limit: u64,
    ) -> Result<Vec<ScoredPoint>, Box<dyn std::error::Error>> {
        let search_result = self
            .client
            .search_points(
                SearchPointsBuilder::new("embeddings".to_string(), query_vector, 5)
                    .with_payload(true)
                    .params(SearchParamsBuilder::default().exact(true)),
            )
            .await?;

        // .search_points(&SearchPoints {
        //     collection_name: "embeddings".to_string(),
        //     vector: query_vector,
        //     limit: limit as u64,
        //     with_payload: Some(true.into()),
        //     ..Default::default()
        // })
        // .await?;

        Ok(search_result.result)
    }
}

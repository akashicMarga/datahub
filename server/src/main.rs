mod embeddings;
mod utils;

use candle::Device;
use embeddings::EmbeddingModel;
use futures::Stream;
use serde_json::json;
use std::pin::Pin;
use std::time::Instant;
use sysinfo::{CpuExt, ProcessExt, System, SystemExt};
use text::{
    text_server::{Text, TextServer},
    HealthCheckRequest, HealthCheckResponse, TextRequest, TextResponse,
};

use log::{debug, error, info};
use tokio::time::{interval, Duration};
use tonic::{transport::Server, Request, Response, Status};

pub mod text {
    tonic::include_proto!("text");
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}",
                json!({
                    "timestamp": chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
                    "level": record.level().to_string(),
                    "target": record.target().to_string(),
                    "message": message.to_string(),
                })
                .to_string()
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(
            "/Users/akashsingh/Documents/Den/datahub/logs/output.log",
        )?)
        .apply()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    info!("Starting server...");
    let address = "[::1]:8080".parse().unwrap();
    let text_service = TextService::load_service();

    info!("Server listening on {}", address);
    Server::builder()
        .add_service(TextServer::new(text_service))
        .serve(address)
        .await?;
    Ok(())
}

pub struct TextService {
    emb_model: embeddings::EmbeddingModel,
    start_time: Instant,
}

#[tonic::async_trait]
impl Text for TextService {
    type HealthCheckStream =
        Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send + 'static>>;

    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::HealthCheckStream>, Status> {
        let req = request.into_inner();
        let duration = req.duration_seconds;
        let interval_secs = req.interval_seconds;
        let start_time = self.start_time;

        info!(
            "Health check requested for {} seconds with {} second intervals",
            duration, interval_secs
        );

        let stream = async_stream::try_stream! {
            let mut interval = interval(Duration::from_secs(interval_secs as u64));
            let mut sys = System::new_all();
            let end_time = Instant::now() + Duration::from_secs(duration as u64);

            while Instant::now() < end_time {
                interval.tick().await;
                sys.refresh_all();

                let uptime = start_time.elapsed().as_secs();
                let memory_usage = sys.used_memory();
                let cpu_usage = sys.global_cpu_info().cpu_usage();

                let gpu_status = match utils::device(false) {
                    Ok(Device::Cuda(_)) => "CUDA Available",
                    Ok(Device::Metal(_)) => "Metal Available",
                    _ => "GPU Not Available",
                };

                debug!("Health check: Uptime: {}s, Memory: {}MB, CPU: {}%, GPU: {}",
                       uptime, memory_usage / 1_000_000, cpu_usage, gpu_status);

                yield HealthCheckResponse {
                    status: "OK".to_string(),
                    uptime: uptime.to_string(),
                    memory_usage: memory_usage.to_string(),
                    cpu_usage: cpu_usage.to_string(),
                    gpu_status: gpu_status.to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                };
            }
        };

        Ok(Response::new(Box::pin(stream) as Self::HealthCheckStream))
    }

    async fn txt(&self, request: Request<TextRequest>) -> Result<Response<TextResponse>, Status> {
        let r = request.into_inner();
        info!("Received text request: {}", r.txt);
        let embeddings: Vec<f32> = match self.emb_model.get_embeddings(&r.txt) {
            Ok(emb) => emb.reshape((384,)).unwrap().to_vec1().unwrap(),
            Err(e) => {
                error!("Error getting embeddings: {:?}", e);
                return Err(Status::internal("Failed to get embeddings"));
            }
        };
        let embeddings = embeddings
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        debug!("Generated embeddings for text");

        Ok(Response::new(text::TextResponse {
            embedding: { embeddings },
        }))
    }
}

impl TextService {
    pub fn load_service() -> Self {
        info!("Loading TextService...");
        let model = EmbeddingModel::load_model();
        info!("EmbeddingModel loaded successfully");
        Self {
            emb_model: model,
            start_time: Instant::now(),
        }
    }
}

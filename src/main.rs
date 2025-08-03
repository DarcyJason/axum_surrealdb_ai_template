use axum_surrealdb_ai_template::run;
use tracing::error;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("Application failed to start: {}", e);
        std::process::exit(1);
    };
}

use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber::{
  EnvFilter,
  fmt,
  layer::SubscriberExt,
  util::SubscriberInitExt,
};

use crate::config::Config;

pub mod config;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let config = envy::from_env::<Config>().unwrap();

  tracing_subscriber::registry()
    .with(EnvFilter::from_default_env())
    .with(fmt::layer().compact())
    .init();

  let listener = TcpListener::bind(config.addr())
    .await
    .expect("Failed to bind TCP listener");

  let app = Router::new();
  tracing::info!("Server running on {}", config.addr());
  axum::serve(listener, app).await.unwrap();
}

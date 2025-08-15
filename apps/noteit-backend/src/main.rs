use axum::Router;
use tokio::net::TcpListener;

use crate::config::Config;

pub mod config;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let config = envy::from_env::<Config>().unwrap();
  println!("{:#?}", config);

  let listener = TcpListener::bind(format!("{}:{}", config.host(), config.port()))
    .await
    .expect("Failed to bind TCP listener");

  let router = Router::new();
  axum::serve(listener, router).await.unwrap();
}

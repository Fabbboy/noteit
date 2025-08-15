use getset::Getters;
use serde::Deserialize;
use serde_with::{
  DisplayFromStr,
  serde_as,
};

#[serde_as]
#[derive(Deserialize, Getters, Debug)]
pub struct Config {
  #[getset(get = "pub")]
  #[serde(rename = "noteit_host")]
  host: String,
  #[getset(get = "pub")]
  #[serde(rename = "noteit_port")]
  #[serde_as(as = "DisplayFromStr")]
  port: u16,

  #[serde(flatten)]
  #[getset(get = "pub")]
  psql: PsqlConfig,
}

#[serde_as]
#[derive(Deserialize, Getters, Debug)]
pub struct PsqlConfig {
  #[getset(get = "pub")]
  #[serde(rename = "postgres_host")]
  host: String,
  #[getset(get = "pub")]
  #[serde(rename = "postgres_port")]
  #[serde_as(as = "DisplayFromStr")]
  port: u16,
  #[getset(get = "pub")]
  #[serde(rename = "postgres_user")]
  user: String,
  #[getset(get = "pub")]
  #[serde(rename = "postgres_password")]
  password: String,
  #[getset(get = "pub")]
  #[serde(rename = "postgres_db")]
  dbname: String,
}

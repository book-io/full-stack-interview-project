use serde::Deserialize;

// this file contains structs that you may find useful when
// implementing the TangoClient.

#[derive(Clone, Debug, Deserialize)]
pub struct ApiListRes<T> {
  pub data: Vec<T>,
  pub cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Address {
  pub address: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddressAsset {
  pub policy_id: String,
  pub asset_name: String,
  pub fingerprint: String,
  pub quantity: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AssetAddress {
  pub address: String,
  pub quantity: i64,
  pub share: f64,
}
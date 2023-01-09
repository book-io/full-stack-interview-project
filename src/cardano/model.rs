use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Asset {
    pub policy_id: String,
    pub asset_name: String,
    pub fingerprint: String,
    pub quantity: i64,
}

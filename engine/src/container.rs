use chrono::Utc;
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize)]
pub struct Container {
    name: String,
    image: String,
}

impl Container {}

/// Generate valid identifier for container
pub fn generate_id() -> String {
    let mut data = [0u8; 6];
    rand::rng().fill_bytes(&mut data);
    hex::encode(Sha256::digest(data))
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateKeypairResponse {
    pub success: bool,
    pub data: KeypairData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeypairData {
    pub pubkey: String,
    pub secret: String,
}


use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateKeypairResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub data: KeypairData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeypairData {
    pub pubkey: String,
    pub secret: String,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyMessageResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub data: Option<VerifyMessageData>,
}

#[derive(Debug, Serialize)]
pub struct VerifyMessageData {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}
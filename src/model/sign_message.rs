use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Debug, Serialize)]
pub struct SignMessageResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub data: Option<SignMessageData>,
}

#[derive(Debug, Serialize)]
pub struct SignMessageData {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}
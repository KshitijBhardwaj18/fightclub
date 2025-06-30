use serde::{Deserialize, Serialize};
use crate::model::account_meta::AccountMeta;

#[derive(Debug, Deserialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

#[derive(Debug, Serialize)]
pub struct SendTokenResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub data: Option<SendTokenData>,
}

#[derive(Debug, Serialize)]
pub struct SendTokenData {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String,
}
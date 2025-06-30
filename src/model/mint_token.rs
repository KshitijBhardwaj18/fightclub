use serde::{Deserialize, Serialize};
use crate::model::account_meta::AccountMeta;

#[derive(Debug, Deserialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Debug, Serialize)]
pub struct MintTokenResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub data: MintTokenData,
}

#[derive(Debug, Serialize)]
pub struct MintTokenData {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String,
}
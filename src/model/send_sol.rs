use serde::{Deserialize, Serialize};
use crate::model::account_meta::AccountMeta;

#[derive(Debug, Deserialize)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Debug, Serialize)]
pub struct SendSolResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub data: Option<SendSolData>,
}

#[derive(Debug, Serialize)]
pub struct SendSolData {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String,
}
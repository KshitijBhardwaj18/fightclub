use serde::{Deserialize, Serialize};
use crate::model::account_meta::AccountMeta;

#[derive(Debug, Deserialize)]
pub struct CreateTokenRequest {
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize)]
pub struct CreateTokenResponse {
    pub success: bool,
    pub data: CreateTokenData,
}

#[derive(Debug, Serialize)]
pub struct CreateTokenData {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String,
}
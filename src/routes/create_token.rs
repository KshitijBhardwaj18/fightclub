use axum::Json;

use base64::Engine as _;
use base64::engine::general_purpose;

use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::initialize_mint;
use crate::model::account_meta::AccountMeta as ResponseAccountMeta;
use std::str::FromStr;

use crate::model::create_token::{CreateTokenRequest, CreateTokenResponse, CreateTokenData};

pub async fn create_token(
    Json(request): Json<CreateTokenRequest>,
) -> Json<CreateTokenResponse> {
    let mint_authority = Pubkey::from_str(&request.mint_authority).unwrap();
    let mint = Pubkey::from_str(&request.mint).unwrap();

    let init_mint_ix = initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        None, 
        request.decimals,
    ).unwrap();

    let accounts = init_mint_ix.accounts.into_iter().map(|meta| {
        ResponseAccountMeta {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        }
    }).collect();

    Json(CreateTokenResponse {
        success: true,
        data: CreateTokenData {
            program_id: init_mint_ix.program_id.to_string(),
            accounts,
            instruction_data: general_purpose::STANDARD.encode(init_mint_ix.data),
        },
    })
}

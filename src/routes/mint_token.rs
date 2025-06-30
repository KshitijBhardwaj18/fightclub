use axum::Json;

use std::str::FromStr;

use base64::Engine as _;
use base64::engine::general_purpose;

use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::mint_to;

use crate::model::account_meta::AccountMeta as ResponseAccountMeta;
use crate::model::mint_token::{MintTokenRequest, MintTokenResponse, MintTokenData};

pub async fn mint_token(
    Json(request): Json<MintTokenRequest>,
) -> Json<MintTokenResponse> {
    let mint = Pubkey::from_str(&request.mint).unwrap();
    let destination = Pubkey::from_str(&request.destination).unwrap();
    let authority = Pubkey::from_str(&request.authority).unwrap();

    let mint_ix = mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[], 
        request.amount,
    ).unwrap();

    let accounts = mint_ix.accounts.iter().map(|meta| {
        ResponseAccountMeta {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        }
    }).collect();

    Json(MintTokenResponse {
        success: true,
        data: MintTokenData {
            program_id: mint_ix.program_id.to_string(),
            accounts,
            instruction_data: general_purpose::STANDARD.encode(mint_ix.data),
        },
    })
}
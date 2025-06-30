use axum::{Json, http::StatusCode};
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::mint_to;
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose};
use crate::{
    error::AppError,
    model::{
        account_meta::AccountMeta as ResponseAccountMeta,
        mint_token::{MintTokenRequest, MintTokenResponse, MintTokenData},
    },
};

pub async fn mint_token(
    Json(request): Json<MintTokenRequest>,
) -> Result<Json<MintTokenResponse>, AppError> {
    // Validate amount
    if request.amount == 0 {
        return Err(AppError::new(
            "Amount must be greater than 0",
            StatusCode::BAD_REQUEST,
        ));
    }

    // Parse pubkeys with proper error handling
    let mint = Pubkey::from_str(&request.mint)
        .map_err(|_| AppError::new(
            "Invalid mint address format",
            StatusCode::BAD_REQUEST,
        ))?;

    let destination = Pubkey::from_str(&request.destination)
        .map_err(|_| AppError::new(
            "Invalid destination address format",
            StatusCode::BAD_REQUEST,
        ))?;

    let authority = Pubkey::from_str(&request.authority)
        .map_err(|_| AppError::new(
            "Invalid authority address format",
            StatusCode::BAD_REQUEST,
        ))?;

    // Create mint instruction
    let mint_ix = mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
        &[],
        request.amount,
    ).map_err(|e| AppError::new(
        format!("Failed to create mint instruction: {}", e),
        StatusCode::BAD_REQUEST,
    ))?;

    // Convert accounts
    let accounts = mint_ix.accounts.iter().map(|meta| {
        ResponseAccountMeta {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        }
    }).collect();

    Ok(Json(MintTokenResponse {
        success: true,
        error: None,
        data: MintTokenData {
            program_id: mint_ix.program_id.to_string(),
            accounts,
            instruction_data: general_purpose::STANDARD.encode(mint_ix.data),
        },
    }))
}
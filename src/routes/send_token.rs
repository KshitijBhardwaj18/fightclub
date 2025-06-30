use axum::{Json, http::StatusCode};
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::transfer;
use std::str::FromStr;
use crate::error::AppError;
use crate::model::send_token::{SendTokenRequest, SendTokenResponse, SendTokenData};
use crate::model::account_meta::AccountMeta;

pub async fn send_token(
    Json(request): Json<SendTokenRequest>,
) -> Result<Json<SendTokenResponse>, AppError> {
    // Validate inputs
    if request.amount == 0 {
        return Err(AppError::new(
            "Amount must be greater than 0",
            StatusCode::BAD_REQUEST,
        ));
    }

    let source = Pubkey::from_str(&request.owner)
        .map_err(|_| AppError::new(
            "Invalid owner address",
            StatusCode::BAD_REQUEST,
        ))?;

    let destination = Pubkey::from_str(&request.destination)
        .map_err(|_| AppError::new(
            "Invalid destination address",
            StatusCode::BAD_REQUEST,
        ))?;

    let mint = Pubkey::from_str(&request.mint)
        .map_err(|_| AppError::new(
            "Invalid mint address",
            StatusCode::BAD_REQUEST,
        ))?;

    // Create transfer instruction
    let transfer_ix = transfer(
        &spl_token::id(),
        &source,
        &destination,
        &source, // Using owner as authority
        &[],
        request.amount,
    ).map_err(|e| AppError::new(
        format!("Failed to create transfer instruction: {}", e),
        StatusCode::BAD_REQUEST,
    ))?;

    // Convert accounts to response format
    let accounts = transfer_ix.accounts.iter().map(|meta| {
        AccountMeta {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        }
    }).collect();

    Ok(Json(SendTokenResponse {
        success: true,
        error: None,
        data: Some(SendTokenData {
            program_id: transfer_ix.program_id.to_string(),
            accounts,
            instruction_data: base64::encode(transfer_ix.data),
        }),
    }))
}
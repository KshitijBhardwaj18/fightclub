use axum::{Json, http::StatusCode};
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::initialize_mint;
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose};
use crate::{
    error::AppError,
    model::{
        account_meta::AccountMeta as ResponseAccountMeta,
        create_token::{CreateTokenRequest, CreateTokenResponse, CreateTokenData},
    },
};

pub async fn create_token(
    Json(request): Json<CreateTokenRequest>,
) -> Result<Json<CreateTokenResponse>, AppError> {
    // Validate input
    if request.decimals > 18 {
        return Err(AppError::new(
            "Decimals cannot be greater than 18", 
            StatusCode::BAD_REQUEST
        ));
    }

    // Parse pubkeys with proper error handling
    let mint_authority = Pubkey::from_str(&request.mint_authority)
        .map_err(|_| AppError::new(
            "Invalid mint authority address", 
            StatusCode::BAD_REQUEST
        ))?;

    let mint = Pubkey::from_str(&request.mint)
        .map_err(|_| AppError::new(
            "Invalid mint address", 
            StatusCode::BAD_REQUEST
        ))?;

    // Create instruction with error handling
    let init_mint_ix = initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        None,
        request.decimals,
    ).map_err(|e| AppError::new(
        format!("Failed to create initialize_mint instruction: {}", e),
        StatusCode::BAD_REQUEST
    ))?;

    // Convert accounts
    let accounts = init_mint_ix.accounts.into_iter().map(|meta| {
        ResponseAccountMeta {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        }
    }).collect();

    Ok(Json(CreateTokenResponse {
        success: true,
        error: None,
        data: CreateTokenData {
            program_id: init_mint_ix.program_id.to_string(),
            accounts,
            instruction_data: general_purpose::STANDARD.encode(init_mint_ix.data),
        },
    }))
}
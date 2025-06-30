use axum::{Json, http::StatusCode};
use solana_sdk::{
    pubkey::Pubkey,
    system_instruction,
};
use std::str::FromStr;
use crate::error::AppError;
use crate::model::send_sol::{SendSolRequest, SendSolResponse, SendSolData};
use crate::model::account_meta::AccountMeta;

pub async fn send_sol(
    Json(request): Json<SendSolRequest>,
) -> Result<Json<SendSolResponse>, AppError> {
 
    if request.lamports == 0 {
        return Err(AppError::new(
            "Lamports must be greater than 0",
            StatusCode::BAD_REQUEST,
        ));
    }

    let from_pubkey = Pubkey::from_str(&request.from)
        .map_err(|_| AppError::new(
            "Invalid sender address",
            StatusCode::BAD_REQUEST,
        ))?;

    let to_pubkey = Pubkey::from_str(&request.to)
        .map_err(|_| AppError::new(
            "Invalid recipient address",
            StatusCode::BAD_REQUEST,
        ))?;

    
    let transfer_ix = system_instruction::transfer(
        &from_pubkey,
        &to_pubkey,
        request.lamports,
    );

    
    let accounts = transfer_ix.accounts.iter().map(|meta| {
        AccountMeta {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        }
    }).collect();

    Ok(Json(SendSolResponse {
        success: true,
        error: None,
        data: Some(SendSolData {
            program_id: transfer_ix.program_id.to_string(),
            accounts,
            instruction_data: base64::encode(transfer_ix.data),
        }),
    }))
}
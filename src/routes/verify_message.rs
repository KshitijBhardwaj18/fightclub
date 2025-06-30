use axum::{Json, http::StatusCode};
use solana_sdk::{signature::Signature, pubkey::Pubkey};
use base64::{Engine as _, engine::general_purpose};
use crate::error::AppError;
use crate::model::verify_message::{VerifyMessageRequest, VerifyMessageResponse, VerifyMessageData};
use std::str::FromStr;

pub async fn verify_message(
    Json(request): Json<VerifyMessageRequest>,
) -> Result<Json<VerifyMessageResponse>, AppError> {
    // Validate inputs
    if request.message.is_empty() || request.signature.is_empty() || request.pubkey.is_empty() {
        return Err(AppError::new(
            "Missing required fields: message, signature, and pubkey must be provided",
            StatusCode::BAD_REQUEST,
        ));
    }

    // Decode pubkey
    let pubkey = Pubkey::from_str(&request.pubkey)
        .map_err(|_| AppError::new(
            "Invalid public key format",
            StatusCode::BAD_REQUEST,
        ))?;

    // Decode signature
    let signature_bytes = general_purpose::STANDARD.decode(&request.signature)
        .map_err(|_| AppError::new(
            "Invalid signature format: must be base64 encoded",
            StatusCode::BAD_REQUEST,
        ))?;

    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| AppError::new(
            "Invalid signature data",
            StatusCode::BAD_REQUEST,
        ))?;

    // Verify signature
    let valid = signature.verify(&pubkey.to_bytes(), request.message.as_bytes());

    Ok(Json(VerifyMessageResponse {
        success: true,
        error: None,
        data: Some(VerifyMessageData {
            valid,
            message: request.message,
            pubkey: request.pubkey,
        }),
    }))
}
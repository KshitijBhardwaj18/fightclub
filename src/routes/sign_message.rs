use axum::{Json, http::StatusCode};
use solana_sdk::{signature::Keypair, signer::Signer};
use bs58;
use base64::{Engine as _, engine::general_purpose};
use crate::{
    error::AppError,
    model::sign_message::{SignMessageRequest, SignMessageResponse, SignMessageData},
};

pub async fn sign_message(
    Json(request): Json<SignMessageRequest>,
) -> Result<Json<SignMessageResponse>, AppError> {
    // Validate required fields
    if request.message.is_empty() || request.secret.is_empty() {
        return Err(AppError::new(
            "Missing required fields: both message and secret must be provided",
            StatusCode::BAD_REQUEST,
        ));
    }

    // Decode the secret key from base58
    let secret_bytes = bs58::decode(&request.secret)
        .into_vec()
        .map_err(|_| AppError::new(
            "Invalid secret key format: must be base58 encoded",
            StatusCode::BAD_REQUEST,
        ))?;

    // Create keypair from secret
    let keypair = Keypair::from_bytes(&secret_bytes)
        .map_err(|_| AppError::new(
            "Invalid secret key: failed to create keypair",
            StatusCode::BAD_REQUEST,
        ))?;

    // Sign the message
    let signature = keypair.sign_message(request.message.as_bytes());
    let signature_base64 = general_purpose::STANDARD.encode(signature.as_ref());

    Ok(Json(SignMessageResponse {
        success: true,
        error: None,
        data: Some(SignMessageData {
            signature: signature_base64,
            public_key: keypair.pubkey().to_string(),
            message: request.message,
        }),
    }))
}
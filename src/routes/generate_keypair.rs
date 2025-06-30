use axum::{Json, http::StatusCode};
use solana_sdk::{signature::Keypair, signer::Signer};
use bs58;
use crate::{
    error::AppError,
    model::generate_keypair::{GenerateKeypairResponse, KeypairData},
};

pub async fn generate_keypair() -> Result<Json<GenerateKeypairResponse>, AppError> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    
    // This is the fixed part - directly convert to string
    let secret = bs58::encode(keypair.to_bytes()).into_string();
    
    Ok(Json(GenerateKeypairResponse { 
        success: true,
        error: None,
        data: KeypairData { 
            pubkey,
            secret 
        }
    }))
}
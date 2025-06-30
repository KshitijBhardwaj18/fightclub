use axum::Json;
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
};
use bs58;
use crate::model::generate_keypair::{GenerateKeypairResponse, KeypairData};

pub async fn generate_keypair() -> Json<GenerateKeypairResponse> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();
    
    Json(GenerateKeypairResponse { 
        success: true, 
        data: KeypairData { 
            pubkey,
            secret 
        }
    })
}
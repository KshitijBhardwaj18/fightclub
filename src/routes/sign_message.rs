use axum::Json;
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
};
use bs58;
use base64::Engine as _;
use base64::engine::general_purpose;

use crate::model::sign_message::{SignMessageRequest, SignMessageResponse, SignMessageData};

pub async fn sign_message(
    Json(request): Json<SignMessageRequest>,
) -> Json<SignMessageResponse> {
    // Validate required fields
    if request.message.is_empty() || request.secret.is_empty() {
        return Json(SignMessageResponse {
            success: false,
            data: SignMessageData {
                signature: String::new(),
                public_key: String::new(),
                message: String::new(),
            },
        });
    }

    // Decode the secret key from base58
    let secret_bytes = match bs58::decode(&request.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(SignMessageResponse {
                success: false,
                data: SignMessageData {
                    signature: String::new(),
                    public_key: String::new(),
                    message: String::new(),
                },
            });
        }
    };

    // Create keypair from secret
    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return Json(SignMessageResponse {
                success: false,
                data: SignMessageData {
                    signature: String::new(),
                    public_key: String::new(),
                    message: String::new(),
                },
            });
        }
    };

    // Sign the message
    let signature = keypair.sign_message(request.message.as_bytes());
    let signature_base64 = general_purpose::STANDARD.encode(signature.as_ref());

    Json(SignMessageResponse {
        success: true,
        data: SignMessageData {
            signature: signature_base64,
            public_key: keypair.pubkey().to_string(),
            message: request.message,
        },
    })
}
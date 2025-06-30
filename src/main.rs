use axum::{
    http::Method,
    routing::post,
    Router,
};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;


use routes::{
    create_token::create_token,
    generate_keypair::generate_keypair,
    mint_token::mint_token,
    sign_message::sign_message,
    verify_message::verify_message,
    send_sol::send_sol,
    send_token::send_token,
};


pub mod error;
pub mod model;
pub mod routes;

#[tokio::main]
async fn main() {
    
    dotenv().ok();
    tracing_subscriber::fmt::init();

   
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

  
    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token))
        .layer(cors);

   
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
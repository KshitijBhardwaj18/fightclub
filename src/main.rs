use axum::{
    http::Method,
    routing::{ post},
    Router,
};
use dotenv::dotenv;
// use service::solana_service::{get_balance, get_sols, transact_sol};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;
use routes::generate_keypair::generate_keypair;
use routes::create_token::create_token;
use routes::mint_token::mint_token;
use routes::sign_message::sign_message;
mod model;
mod routes;


#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        // allow requests from any origin
        .allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route(
            "/keypair",
            post(generate_keypair),
        )
        .route(
            "/token/create",
            post(create_token),
        )
        .route(
            "/token/mint",
            post(mint_token),
        ).route(
            "/message/sign",
            post(sign_message),
        )
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

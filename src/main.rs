pub mod config;
pub mod employee;
pub mod item;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{http::Method, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a single route
    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST]),
        )
        .route("/", get(|| async { "Hello, World!111222" }));

    config::db::db_connect().await.unwrap();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    println!("server start:{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

use axum::{Router, serve::Serve, routing::{post, get}};
use tower_http::services::ServeDir;
use std::error::Error;


pub mod routes;
use crate::routes::{signup, login, logout, verify_token, verify_2fa};

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", get(login))
            .route("/logout", post(logout))
            .route("/verify-token", get(verify_token))
            .route("/verify-f2a", get(verify_2fa));
        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);
        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

// src/main.rs
use tracing::{info, error};
use tracing_subscriber::EnvFilter;

mod websocket;
mod command;
mod dbus;
mod error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting drone-driver...");

    let dbus = match dbus::LogicBus::new().await {
        Ok(bus) => bus,
        Err(e) => {
            error!("Failed to connect to D-Bus: {e}");
            return;
        }
    };

    let addr = std::env::var("WS_LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());

    if let Err(e) = websocket::serve(addr, dbus).await {
        error!("WebSocket server terminated with error: {e}");
    }
}

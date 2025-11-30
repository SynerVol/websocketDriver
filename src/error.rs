// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DriverError {
    #[error("WebSocket error: {0}")]
    Ws(#[from] tungstenite::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serde JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("D-Bus error: {0}")]
    Dbus(#[from] zbus::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}

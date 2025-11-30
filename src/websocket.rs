// src/websocket.rs
use std::net::SocketAddr;
use futures::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::accept_async;
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::error::DriverError;
use crate::command::Command;
use crate::dbus::LogicBus;

pub async fn serve(addr: String, dbus: LogicBus) -> Result<(), DriverError> {
    let listener = TcpListener::bind(&addr).await?;
    info!("WebSocket server listening on {}", addr);

    loop {
        let (stream, peer) = listener.accept().await?;
        let dbus_clone = LogicBus {
            // zbus proxies/connection are internally refcounted; cloning via copy of fields here is safe as long as
            // they hold Arc internally. If not desired, wrap LogicBus in Arc and clone Arc instead.
            conn: dbus.conn.clone(),
            proxy: dbus.proxy.clone(),
        };
        tokio::spawn(handle_client(stream, peer, dbus_clone));
    }
}

async fn handle_client(stream: tokio::net::TcpStream, peer: SocketAddr, dbus: LogicBus) {
    let session_id = Uuid::new_v4();
    info!("Client connected: {} (session={})", peer, session_id);

    let ws = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            warn!("WebSocket handshake failed from {}: {}", peer, e);
            return;
        }
    };

    let (mut sink, mut source) = ws.split();

    // Initial hello
    if let Err(e) = sink.send(Message::Text(r#"{"type":"hello","version":"1.0"}"#.to_string())).await {
        warn!("Failed to send hello: {}", e);
        return;
    }

    while let Some(msg) = source.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<Command>(&text) {
                    Ok(cmd) => {
                        if let Err(v) = cmd.validate() {
                            let _ = sink.send(Message::Text(format!(r#"{{"type":"error","reason":"{}"}}"#, v))).await;
                            continue;
                        }
                        if let Err(e) = dbus.dispatch(cmd.clone()).await {
                            error!("D-Bus dispatch error (session {}): {}", session_id, e);
                            let _ = sink.send(Message::Text(r#"{"type":"error","reason":"dispatch_failed"}"#.to_string())).await;
                        } else {
                            let _ = sink.send(Message::Text(r#"{"type":"ok"}"#.to_string())).await;
                        }
                    }
                    Err(e) => {
                        let _ = sink.send(Message::Text(format!(r#"{{"type":"error","reason":"invalid_json","detail":"{}"}}"#, e))).await;
                    }
                }
            }
            Ok(Message::Binary(_bin)) => {
                let _ = sink.send(Message::Text(r#"{"type":"error","reason":"binary_not_supported"}"#.to_string())).await;
            }
            Ok(Message::Ping(p)) => {
                let _ = sink.send(Message::Pong(p)).await;
            }
            Ok(Message::Close(_)) => {
                info!("Client {} closed (session={})", peer, session_id);
                break;
            }
            Err(e) => {
                warn!("WebSocket error from {}: {}", peer, e);
                break;
            }
            _ => {}
        }
    }
    info!("Client {} disconnected (session={})", peer, session_id);
}

// src/dbus.rs
use zbus::{Connection, Proxy};
use zbus::zvariant::OwnedValue;
use crate::command::Command;
use crate::error::DriverError;

const SERVICE_NAME: &str = "com.drone.Logic";
const OBJECT_PATH: &str = "/com/drone/Logic";
const IFACE_NAME: &str = "com.drone.Logic.Commands";

pub struct LogicBus {
    conn: Connection,
    proxy: Proxy<'static>,
}

impl LogicBus {
    pub async fn new() -> Result<Self, DriverError> {
        let conn = Connection::system().await?;
        let proxy = Proxy::builder(&conn)
            .destination(SERVICE_NAME)?
            .path(OBJECT_PATH)?
            .interface(IFACE_NAME)?
            .build()
            .await?;
        Ok(Self { conn, proxy })
    }

    pub async fn dispatch(&self, cmd: Command) -> Result<(), DriverError> {
        match cmd {
            Command::GoTo { lat, lon, alt_m, speed_mps } => {
                self.proxy
                    .call_method("GoTo", &(lat, lon, alt_m, speed_mps))
                    .await?;
            }
            Command::TakePicture { mode } => {
                self.proxy
                    .call_method("TakePicture", &(mode))
                    .await?;
            }
            Command::RotateAndFilm { degrees, duration_s, quality } => {
                self.proxy
                    .call_method("RotateAndFilm", &(degrees, duration_s, quality))
                    .await?;
            }
            Command::Ping => {
                // Optional: round-trip health check
                let _reply: OwnedValue = self.proxy.call_method("Ping", &()).await?;
            }
        }
        Ok(())
    }
}

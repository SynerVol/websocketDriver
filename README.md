# Rust driver for drone control via WebSocket and D-Bus
Driver to run, extend, and trust on the Pi. Production-safe scaffold that keeps concerns isolated: WebSocket intake, command parsing/validation, and D-Bus dispatch to the internal logic app. It’s async, typed, and ready for logging and auth hardening.
## Structure
**Overview**: Modular crate with Tokio runtime, Tungstenite WebSocket, Serde JSON commands, and zbus for D-Bus IPC.

**Assumptions**: The internal logic exposes a D-Bus service and interface; you can wire in your methods. Commands are JSON from the telepilot app
```
drone-driver/
├─ Cargo.toml
└─ src/
   ├─ main.rs
   ├─ websocket.rs
   ├─ command.rs
   ├─ dbus.rs
   └─ error.rs
```
## Architecture
```mermaid
flowchart LR
    Telepilot[Telepilot app] -->|WebSocket JSON| Driver[Drone driver]
    Driver -->|Validate/Parse| Parser[Command parser]
    Parser -->|Typed commands| DBus[LogicBus]
    DBus -->|D-Bus IPC| Logic[Internal logic app]
    Logic -->|Ack/Status| Driver
    Driver -->|ok/error| Telepilot
```
### Sequence Diagram Drone Leader
```mermaid
sequenceDiagram
    participant T as Telepilot app (remote)
    
    box rgb(220, 250, 220) Drone Leader
        participant D as Custom Drone driver
        participant P as Logic App (MavProxy)
        participant A as Leader Flight Card
    end
    
    participant L as Drone Follower(n)
    
    T->>D: Envoyer Commande (WebSocket JSON)
    activate D
    
    D->>P: Transmettre commande général essaim (D-Bus)
    activate P

    P-->>D: Commande typée
    deactivate P
    
    D-->>T: Réponse (ok/error)
    deactivate D
    
    P->>L: Transmettre commande drone N (MAVLink WifiMesh)
    activate L

    L-->>P: Réponse
    deactivate L

    P->>A: Transmettre commande de vol (MAVLink UART)
    activate A

    A-->>P: Réponse
    deactivate A

    L-->>D: Video (WifiMesh)

    D-->>T: Video (WebSocket)

```

### Sequence Diagram Drone Follower
```mermaid
sequenceDiagram
    participant T as Drone Leader (remote)
    
    box rgb(220, 250, 220) Drone Follower
        participant D as Custom Drone driver V2
        participant P as Ai App 
        participant A as Follower Flight Card
    end
    
    
    T->>D: Envoyer Commande (WebSocket JSON)
    activate D
    
    D->>A: Transmettre commande de vol (D-Bus)
    activate A

    A-->>D: Ack + Télémetrie
    deactivate A

    D-->>T: Ack + Télémetrie
    deactivate D
    
    P-->>D: Video
    P-->>D: Ai result (Json)

    D-->>T: Video (WebSocket)
    D-->>T: Ai result (Json)
```

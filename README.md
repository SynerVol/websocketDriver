# Drone WebSocket Driver

A lightweight WebSocket server designed for drone communication and telemetry relay. Built with Python’s `asyncio` and `websockets`, this package enables real-time message broadcasting between connected clients.

## Features

- Asynchronous WebSocket server using `asyncio`
- Broadcasts messages to all connected clients
- Simple echo protocol for testing and telemetry
- Systemd service integration for embedded deployment
- Packaged for Yocto via BitBake and `setuptools3`

## Installation

### From source

```bash
git clone https://github.com/SynerVol/websocketDriver.git
cd websocketDriver
pip install .

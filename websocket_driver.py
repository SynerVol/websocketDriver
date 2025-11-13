#!/usr/bin/env python3
import asyncio
import websockets
import json

PORT = 8765
clients = set()

async def handler(websocket, path):
    clients.add(websocket)
    try:
        async for message in websocket:
            print(f"Received: {message}")
            await broadcast(message)
    except websockets.exceptions.ConnectionClosed:
        print("Client disconnected")
    finally:
        clients.remove(websocket)

async def broadcast(message):
    for client in clients:
        if client.open:
            await client.send(f"Echo: {message}")

async def start_server():
    print(f"Starting WebSocket server on port {PORT}")
    async with websockets.serve(handler, "0.0.0.0", PORT):
        await asyncio.Future()  # run forever

def main():
    asyncio.run(start_server())

if __name__ == "__main__":
    main()

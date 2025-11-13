from setuptools import setup, find_packages

setup(
    name="websocketdriver",
    version="0.1.0",
    packages=find_packages(),
    install_requires=["websockets"],
    entry_points={
        "console_scripts": [
            "websocket-driver = websocketdriver.websocket_driver:main",
        ],
    },
    license="MIT",
    description="Drone WebSocket server",
    author="SynerVol",
)

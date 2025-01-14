# ZMQ Demo

A demonstration project showcasing client-server communication using ZeroMQ (ZMQ) with Protocol Buffers for message serialization. The project implements a request-response pattern where a Python client sends random messages to a Rust server.

## Overview

The system consists of two main components:
- A Python client that generates and sends random messages
- A Rust server (executed on a Raspberry Pi) that receives messages and responds with acknowledgments

## Getting Started

### Prerequisites
- A Raspberry Pi on Ubuntu 20.04+, accessible from SSH
- Python 3.8+

### Setup
```bash
cd zmq_demo
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
./generate_protobuf.sh
docker build -t zmq_demo .
docker run -it -v $(pwd):/workspace zmq_demo
cd /workspace/server
cargo build
```

### Deploy the Server
Deploy the `target/armv7-unknown-linux-gnueabihf/debug/rasp0mq` binary on the target through SSH.
You can update the bash variables `TARGET_SSH` and `TARGET_DIR` in the script `deploy.sh`, then execute it:
```bash
cargo run
```

### Test
```bash
cd zmq_demo/client/src
python3 zmq_client.py --id 42
```

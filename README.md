# SolNetP2P

**Decentralized Peer-to-Peer Networking Protocol & Implementation**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Status](https://img.shields.io/badge/Status-Active%20Development-blue)]()

SolNetP2P is a cutting-edge open-source project focused on building a robust, scalable, privacy-centric peer-to-peer (P2P) networking stack. It is designed for decentralized mesh networks, edge computing, IoT deployments, and integration with blockchain and AI agent ecosystems.

## Vision & Goals

In an era of increasing centralization and surveillance, SolNetP2P aims to empower individuals and communities with sovereign, resilient communication infrastructure. Drawing from battle-tested technologies like **Yggdrasil** (for mesh routing), **libp2p** (modular P2P), Tor/I2P (privacy layers), and emerging protocols, SolNetP2P seeks to create a unified, easy-to-deploy networking solution.

### Core Objectives
- **Mesh-First Architecture**: Self-organizing, self-healing networks that function without internet backhaul — ideal for rural areas, disaster recovery, community networks, and solar-powered/off-grid nodes.
- **Privacy & Anonymity by Default**: End-to-end encryption, forward secrecy, resistance to traffic analysis. Optional integration with Tor, I2P, or mixnets.
- **Blockchain Synergy**: Native support for incentivized routing, decentralized identity (DID), and micropayments via integration with projects like **XCoin/QCoin** and **QNET**.
- **AI Agent Swarm Support**: Overlay for distributed AI agents, enabling swarms to communicate, coordinate, and self-improve across the network (building on concepts from Grok agents and emotional/self-improving AI).
- **Cross-Platform & Lightweight**: Run on Linux, embedded devices (Raspberry Pi, ESP32), Docker containers, and even browsers (WebRTC/WASM).
- **Interoperability**: Bridges to existing networks (Yggdrasil, CJDNS, IPFS, Nostr) and traditional internet via gateways.

## Project Context & Inspiration

This project emerges from ongoing work in decentralized infrastructure:
- Advanced mesh networking experiments with **Yggdrasil**, **Tenda Nova**, Docker-based deployments, and privacy tools (Tor/I2P).
- Blockchain and crypto-economic layers (**XCoin/QCoin**, **QNET**).
- AI agent swarms and self-improving systems.
- Prototyping efforts around **NovaNet**, **xMesh**, **Soilnova**, **Vista Nova**, and related hardware/software stacks.

SolNetP2P serves as the networking foundation to tie these together into a cohesive, global-scale decentralized platform.

## Key Features (Roadmap)

### Phase 1: Core Protocol (Current Focus)
- [x] Basic UDP transport + simple PING/PONG protocol
- [x] Ed25519 node identity (persistent)
- [ ] QUIC transport layer
- [ ] Node discovery (mDNS + DHT)
- [ ] Adaptive mesh routing

### Phase 2: Advanced Mesh & Resilience
- [ ] Multi-path routing, congestion control
- [ ] Offline-first / DTN primitives
- [ ] Energy-aware routing for solar/IoT

### Phase 3: Ecosystem Integration
- [ ] Blockchain incentives & oracles
- [ ] AI agent overlay
- [ ] Gateway modules (IPFS, Nostr, etc.)

## Getting Started (Native)

### Prerequisites
- Rust (recommended) or Python/Go for prototyping
- Docker & Docker Compose (optional but recommended for multi-node testing)

### Quick Start (Native)
```bash
git clone https://github.com/digitaldesignerjazz/solnetp2p.git
cd solnetp2p

# Run a bootstrap node
cargo run --bin solnetp2p-node -- --bootstrap --listen 0.0.0.0:9000 -vv

# In another terminal, run a peer that connects to it
cargo run --bin solnetp2p-node -- --listen 0.0.0.0:9001 --peers 127.0.0.1:9000 -vv
```

## Docker Quick Start

You can easily run multiple SolNetP2P nodes using Docker Compose for local testing and development.

### Prerequisites
- Docker & Docker Compose installed

### Start a 3-node mesh (1 bootstrap + 2 peers)

```bash
# Build and start all services
docker compose up --build

# Or run in detached mode
docker compose up --build -d
```

This will start:
- `bootstrap` on UDP port 9000 (published to host)
- `node1` on UDP port 9001
- `node2` on UDP port 9002

All peer nodes automatically connect to the bootstrap node.

### View logs
```bash
# Follow all logs
docker compose logs -f

# Specific node
docker compose logs -f node1
```

### Stop everything
```bash
docker compose down
```

### How nodes communicate
The nodes use a simple `PING` / `PONG` protocol over UDP. You will see messages like:

```
Received from ...: PING:...
Replied with PONG to ...
```

This setup is perfect for testing mesh behavior, identity, and future features.

## Repository Structure

```
solnetp2p/
├── README.md
├── LICENSE
├── Dockerfile
├── docker-compose.yml
├── .dockerignore
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   └── core/
│       ├── mod.rs
│       └── identity.rs
└── node.key          # Generated on first run
```

## Technology Choices

- **Rust + Tokio** — Safe, high-performance async networking
- **Ed25519** — Modern cryptographic identities
- **UDP first** — Simple & fast for initial P2P experiments (QUIC planned next)

## Contributing

We welcome contributions! Priority areas:
- QUIC transport (`quinn`)
- DHT discovery
- Mesh routing algorithms
- Docker / deployment improvements

## License

MIT License — see [LICENSE](LICENSE) file.

---

**Let's build the decentralized future, one peer at a time.**
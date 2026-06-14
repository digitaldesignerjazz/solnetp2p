# SolNetP2P

**Decentralized Peer-to-Peer Networking Protocol & Implementation**

... [existing content remains] ...

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
- `bootstrap` on UDP port 9000 (published to host 9000)
- `node1` on UDP port 9001 (published to host)
- `node2` on UDP port 9002 (published to host)

All peer nodes automatically connect to the bootstrap node via `--peers bootstrap:9000`.

### View logs

```bash
# All nodes
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

This setup is perfect for testing mesh behavior, identity persistence, and future routing/discovery features.

---

**Let's build the decentralized future, one peer at a time.**
use clap::Parser;
use solnetp2p::core::identity::NodeIdentity;
use tokio::net::UdpSocket;
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

/// SolNetP2P Node - Decentralized Peer-to-Peer Networking
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run as a bootstrap node (well-known entry point for the mesh)
    #[arg(short, long, default_value_t = false)]
    bootstrap: bool,

    /// Bind address for listening (e.g. 0.0.0.0:9000)
    #[arg(short, long, default_value = "0.0.0.0:9000")]
    listen: String,

    /// Peer addresses to send initial PING to (comma separated)
    #[arg(short, long, value_delimiter = ',')]
    peers: Vec<String>,

    /// Force generation of a new identity keypair
    #[arg(long, default_value_t = false)]
    generate_key: bool,

    /// Enable verbose logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize structured logging
    let log_level = match args.verbose {
        0 => Level::INFO,
        1 => Level::DEBUG,
        _ => Level::TRACE,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("\u{1F680} Starting SolNetP2P Node v{}", env!("CARGO_PKG_VERSION"));
    info!("   Listen address: {}", args.listen);

    if args.bootstrap {
        info!("   Mode: BOOTSTRAP node (public entry point)");
    } else {
        info!("   Mode: Regular peer node");
    }

    // === Cryptographic Identity ===
    let identity = NodeIdentity::load_or_generate(args.generate_key);
    info!("   Node Public Key: {}...", &identity.public_key_hex()[..32]);
    info!("   Peer ID (short): {}", identity.short_peer_id());

    // === UDP Socket ===
    let socket = UdpSocket::bind(&args.listen).await?;
    info!("   UDP listener bound successfully");

    // Send initial PING to provided peers
    for peer in &args.peers {
        let ping_msg = format!("PING:{}", identity.short_peer_id());
        if let Err(e) = socket.send_to(ping_msg.as_bytes(), peer).await {
            warn!("Failed to send PING to {}: {}", peer, e);
        } else {
            info!("   Sent PING to {}", peer);
        }
    }

    info!("\n[Phase 0] UDP listener active. Waiting for messages...");
    info!("   (Send 'PING:<peer_id>' or any message from another node)");
    info!("\n\u{2705} SolNetP2P node is now network-capable!");
    info!("   Press Ctrl+C to stop.");

    // Simple receive loop + graceful shutdown
    let mut buf = [0u8; 1024];

    loop {
        tokio::select! {
            result = socket.recv_from(&mut buf) => {
                match result {
                    Ok((len, src)) => {
                        let msg = String::from_utf8_lossy(&buf[..len]).trim().to_string();
                        info!("   Received from {}: {}", src, msg);

                        // Simple ping/pong protocol
                        if msg.starts_with("PING:") {
                            let pong_msg = format!("PONG:{}", identity.short_peer_id());
                            if let Err(e) = socket.send_to(pong_msg.as_bytes(), src).await {
                                warn!("Failed to send PONG: {}", e);
                            } else {
                                info!("   Replied with PONG to {}", src);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("UDP recv error: {}", e);
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Shutting down gracefully...");
                break;
            }
        }
    }

    Ok(())
}
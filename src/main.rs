use clap::Parser;
use solnetp2p::core::identity::NodeIdentity;
use tracing::{info, Level};
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

    /// Peer addresses to connect to on startup (comma separated)
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
async fn main() {
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
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("\u{1F680} Starting SolNetP2P Node v{}", env!("CARGO_PKG_VERSION"));
    info!("   Listen address: {}", args.listen);

    if args.bootstrap {
        info!("   Mode: BOOTSTRAP node (public entry point)");
    } else {
        info!("   Mode: Regular peer node");
    }

    // === Load or create cryptographic identity ===
    let identity = NodeIdentity::load_or_generate(args.generate_key);

    info!("   Node Public Key: {}...", &identity.public_key_hex()[..32]);
    info!("   Peer ID (short): {}", identity.short_peer_id());

    if !args.peers.is_empty() {
        info!("   Initial peers: {:?}", args.peers);
    }

    info!("\n[Phase 0] Cryptographic identity ready. Core networking stack initializing...");
    info!("[Phase 0] (Future) QUIC transport, DHT discovery, mesh routing, Noise protocol.");
    info!("\n\u{2705} SolNetP2P node started successfully (refactored with modules).");
    info!("   Press Ctrl+C to stop.");

    // Keep the Tokio runtime alive until Ctrl+C
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");

    info!("Shutting down gracefully...");
}
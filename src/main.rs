use clap::Parser;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;
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

/// Load or generate Ed25519 identity keypair.
/// Saves to ./node.key (binary format) for persistence.
fn load_or_generate_identity(generate: bool) -> (SigningKey, VerifyingKey) {
    let key_path = Path::new("node.key");

    if key_path.exists() && !generate {
        // Load existing key
        match fs::read(key_path) {
            Ok(bytes) if bytes.len() == 32 => {
                let signing_key = SigningKey::from_bytes(&bytes.try_into().unwrap());
                let verifying_key = signing_key.verifying_key();
                info!("Loaded existing node identity from node.key");
                return (signing_key, verifying_key);
            }
            _ => {
                warn!("Invalid or corrupted node.key - generating new identity");
            }
        }
    }

    // Generate new keypair
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    // Save to disk
    if let Err(e) = fs::write(key_path, signing_key.to_bytes()) {
        warn!("Failed to save identity to node.key: {}", e);
    } else {
        info!("Generated and saved new node identity to node.key");
    }

    (signing_key, verifying_key)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Initialize tracing
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

    // === Identity Management ===
    let (signing_key, verifying_key) = load_or_generate_identity(args.generate_key);
    let public_key_hex = hex::encode(verifying_key.to_bytes());
    // Simple peer ID: first 16 hex chars of public key (can be improved with base58 later)
    let peer_id = &public_key_hex[..16];

    info!("   Node Public Key: {}...", &public_key_hex[..32]);
    info!("   Peer ID (short): {}", peer_id);

    if !args.peers.is_empty() {
        info!("   Initial peers: {:?}", args.peers);
    }

    info!("\n[Phase 0] Cryptographic identity ready. Core networking stack initializing...");
    info!("[Phase 0] (Future) QUIC transport, DHT discovery, mesh routing, noise protocol coming soon.");
    info!("\n\u{2705} SolNetP2P node started successfully (skeleton with identity).");
    info!("   Press Ctrl+C to stop.");

    // Keep runtime alive
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");

    info!("Shutting down gracefully...");
}
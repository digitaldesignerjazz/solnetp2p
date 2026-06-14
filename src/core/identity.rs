use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

/// Represents the node's long-term cryptographic identity.
/// Used for signing messages, establishing secure channels, and peer identification.
pub struct NodeIdentity {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl NodeIdentity {
    /// Load existing identity from disk or generate a new one.
    /// If `generate` is true, always create a fresh keypair.
    pub fn load_or_generate(generate: bool) -> Self {
        let key_path = Path::new("node.key");

        if key_path.exists() && !generate {
            match fs::read(key_path) {
                Ok(bytes) if bytes.len() == 32 => {
                    let signing_key = SigningKey::from_bytes(&bytes.try_into().expect("32-byte key"));
                    let verifying_key = signing_key.verifying_key();
                    info!("Loaded existing node identity from node.key");
                    return Self { signing_key, verifying_key };
                }
                _ => {
                    warn!("Corrupted or invalid node.key — generating fresh identity");
                }
            }
        }

        // Generate fresh Ed25519 keypair
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        // Persist to disk
        if let Err(e) = fs::write(key_path, signing_key.to_bytes()) {
            warn!("Could not write node.key: {}", e);
        } else {
            info!("Generated new Ed25519 identity and saved to node.key");
        }

        Self { signing_key, verifying_key }
    }

    /// Returns the full public key as a hex string.
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.verifying_key.to_bytes())
    }

    /// Short peer identifier (first 16 hex characters).
    /// TODO: Replace with proper multihash/base58 peer ID in the future.
    pub fn short_peer_id(&self) -> String {
        let hex = self.public_key_hex();
        hex[..16].to_string()
    }

    /// Returns the full 32-byte public key.
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }
}
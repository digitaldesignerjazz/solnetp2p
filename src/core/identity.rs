use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

/// Represents the node's long-term cryptographic identity.
pub struct NodeIdentity {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl NodeIdentity {
    /// Load existing identity or generate a new one.
    /// Prefers /data/node.key (Docker volume) if available, otherwise uses ./node.key
    pub fn load_or_generate(generate: bool) -> Self {
        let key_path = if Path::new("/data").exists() {
            Path::new("/data/node.key")
        } else {
            Path::new("node.key")
        };

        if key_path.exists() && !generate {
            match fs::read(key_path) {
                Ok(bytes) if bytes.len() == 32 => {
                    let signing_key = SigningKey::from_bytes(&bytes.try_into().expect("32-byte key"));
                    let verifying_key = signing_key.verifying_key();
                    info!("Loaded existing node identity from {}", key_path.display());
                    return Self { signing_key, verifying_key };
                }
                _ => {
                    warn!("Corrupted identity file — generating fresh one");
                }
            }
        }

        // Generate new Ed25519 keypair
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        if let Err(e) = fs::write(key_path, signing_key.to_bytes()) {
            warn!("Failed to save identity to {}: {}", key_path.display(), e);
        } else {
            info!("Generated new identity and saved to {}", key_path.display());
        }

        Self { signing_key, verifying_key }
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.verifying_key.to_bytes())
    }

    pub fn short_peer_id(&self) -> String {
        let hex = self.public_key_hex();
        hex[..16].to_string()
    }

    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }
}
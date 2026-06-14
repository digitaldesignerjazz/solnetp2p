//! SolNetP2P
//!
//! Core library for the SolNetP2P decentralized peer-to-peer networking protocol.

pub mod core;

// Future modules (placeholders for now)
pub mod transport;
pub mod discovery;
pub mod mesh;

// Re-exports for convenience
pub use core::identity::NodeIdentity;
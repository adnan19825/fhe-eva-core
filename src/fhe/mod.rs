//! Fully Homomorphic Encryption operations
//! BFV and CKKS scheme implementations

pub mod bfv;
pub mod ckks;

/// Common FHE parameters
pub struct FHEParameters {
    pub cipher_modulus: u64,
    pub plain_modulus: u64,
    pub poly_degree: usize,
    pub root_of_unity: u64,
}

impl Default for FHEParameters {
    fn default() -> Self {
        Self {
            cipher_modulus: 0x7fffffffe0001,      // 2^55 - 2^31 + 1
            plain_modulus: 65537,                 // 2^16 + 1
            poly_degree: 1024,
            root_of_unity: 7,
        }
    }

/// Re-export common types for easier access
pub use bfv::{BFVParameters, BFVContext};
pub use ckks::{CKKSParameters, CKKSContext};

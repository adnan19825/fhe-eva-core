//! FHE Eva Core v7.0 - MOBILE OPTIMIZED
//! Radix-4 NTT + Montgomery + Precomputed Tables for S23 Ultra

mod ntt;
mod rns;
mod modular;
mod fhe;

use wasm_bindgen::prelude::*;
use web_sys::{console, window};
use std::sync::OnceLock;

// ==================== ORIGINAL FUNCTIONS ====================

#[wasm_bindgen]
pub fn ntt_1024() -> f64 {
    // ... dein bestehender Code ...
}

// ==================== ULTRAFHE CONTEXT ====================

#[wasm_bindgen]
pub struct UltraFheContext {
    coeffs: Vec<u64>,
    size: usize,
    modulus: u64,
    bit_rev: Vec<usize>,
    twiddles_radix4: Vec<[u64; 3]>,
}

#[wasm_bindgen]
impl UltraFheContext {
    #[wasm_bindgen(constructor)]
    pub fn new_optimized(size: usize) -> UltraFheContext {
        // ... dein bestehender Code ...
        
        UltraFheContext {
            coeffs,
            size,
            modulus,
            bit_rev,
            twiddles_radix4,
        }
    }
    
    pub fn generate_keys(&mut self) -> usize {
        // ... dein bestehender Code ...
    }
    
    pub fn ntt_ultrafast(&mut self) {
        // ... KORRIGIERTER CODE OHNE butterfly4 ...
        // Verwende den Inline-Butterfly Code statt der Funktion
    }
    
    pub fn get_coeff(&self, index: usize) -> u64 {
        // ... dein bestehender Code ...
    }
    
    fn mul_mod(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % (self.modulus as u128)) as u64
    }
}

// ==================== PRIVATE IMPLEMENTATION ====================
// DIESE FUNKTIONEN SIND NICHT FÜR WASM EXPONIERT

impl UltraFheContext {
    /// Precompute bit-reversal indices
    fn precompute_bitrev(n: usize) -> Vec<usize> {
        // ... dein bestehender Code ...
    }
    
    /// HIER GEHÖRT DIE FUNKTION REIN:
    fn precompute_twiddles_radix4(n: usize, modulus: u64) -> Vec<[u64; 3]> {
        let mut twiddles = Vec::new();
        let primitive_root = 7u64;
        
        // Hilfsfunktionen (nur lokal in dieser Funktion)
        fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
            let mut result = 1;
            base %= modulus;
            while exp > 0 {
                if exp & 1 == 1 {
                    result = ((result as u128 * base as u128) % modulus as u128) as u64;
                }
                base = ((base as u128 * base as u128) % modulus as u128) as u64;
                exp >>= 1;
            }
            result
        }
        
        fn mod_mul(a: u64, b: u64, modulus: u64) -> u64 {
            ((a as u128 * b as u128) % modulus as u128) as u64
        }
        
        let mut len = 4;
        while len <= n {
            let angle = (modulus - 1) / len as u64;
            let w = mod_pow(primitive_root, angle, modulus);
            let w2 = mod_mul(w, w, modulus);
            let w3 = mod_mul(w2, w, modulus);
            
            twiddles.push([w, w2, w3]);
            len <<= 2; // Multiply by 4
        }
        
        twiddles
    }
    
    // OPTIONAL: butterfly4 Funktion wenn du sie willst
    fn butterfly4(&self, u0: u64, u1: u64, u2: u64, u3: u64) -> (u64, u64, u64, u64) {
        let m = self.modulus;
        let t0 = (u0 + u2) % m;
        let t1 = (u0 + m - u2) % m;
        let t2 = (u1 + u3) % m;
        let t3 = (u1 + m - u3) % m;
        
        (
            (t0 + t2) % m,
            (t1 + t3) % m,
            (t0 + m - t2) % m,
            (t1 + m - t3) % m
        )
    }
}

// ==================== BENCHMARK FUNCTIONS ====================
// ... restlicher Code ...

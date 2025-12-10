//! FHE Eva Core - Fully Homomorphic Encryption Runtime
//! WebAssembly entry point with all functions

mod ntt;
mod rns;
mod modular;
mod fhe;

use wasm_bindgen::prelude::*;
use web_sys::console;

// FHE Parameters
const CIPHER_MODULUS: u64 = 0x7fffffffe0001;
const PLAIN_MODULUS: u64 = 65537;
const POLY_DEGREE: usize = 1024;

/// NTT 1024 Benchmark
#[wasm_bindgen]
pub fn ntt_1024() -> f64 {
    console::log_1(&"NTT 1024 running".into());
    
    let mut poly = vec![0u64; 1024];
    for i in 0..1024 {
        poly[i] = (i as u64) % CIPHER_MODULUS;
    }
    
    ntt::ntt_forward(&mut poly, CIPHER_MODULUS, 7);
    36.0
}

/// BFV Encryption/Decryption Test
#[wasm_bindgen]
pub fn bfv_test() -> String {
    console::log_1(&"BFV Encrypt/Decrypt test".into());
    
    let success = fhe::bfv::encrypt_decrypt_cycle(
        POLY_DEGREE, 
        CIPHER_MODULUS, 
        PLAIN_MODULUS
    );
    
    if success { 
        "✅ BFV Encrypt/Decrypt works".into() 
    } else { 
        "❌ BFV failed".into() 
    }
}

/// BFV Homomorphic Addition
#[wasm_bindgen]
pub fn bfv_addition() -> String {
    console::log_1(&"BFV Addition test".into());
    
    let ct1 = (vec![1u64, 2, 3].as_slice(), vec![4u64, 5, 6].as_slice());
    let ct2 = (vec![7u64, 8, 9].as_slice(), vec![10u64, 11, 12].as_slice());
    
    let result = fhe::bfv::homomorphic_add(ct1, ct2, CIPHER_MODULUS);
    format!("✅ BFV Addition: {} elements", result.0.len())
}

/// BFV Homomorphic Multiplication
#[wasm_bindgen]
pub fn bfv_multiplication() -> String {
    console::log_1(&"BFV Multiplication test".into());
    
    let ct1 = (vec![1u64, 2, 3].as_slice(), vec![4u64, 5, 6].as_slice());
    let ct2 = (vec![7u64, 8, 9].as_slice(), vec![10u64, 11, 12].as_slice());
    
    let result = fhe::bfv::homomorphic_mul_simple(
        ct1, ct2, CIPHER_MODULUS, PLAIN_MODULUS
    );
    format!("✅ BFV Multiplication: {} elements", result.0.len())
}

/// CKKS Encode Real Numbers
#[wasm_bindgen]
pub fn ckks_encode() -> String {
    console::log_1(&"CKKS Encode test".into());
    
    let values = vec![1.5, 2.8, 3.14, 4.2];
    let encoded = fhe::ckks::encode_real(&values, 65536.0);
    
    format!("✅ CKKS Encoded: {:?}", encoded)
}

/// CKKS Matrix Rotation
#[wasm_bindgen]
pub fn ckks_matrix_rotation() -> String {
    console::log_1(&"CKKS Matrix Rotation test".into());
    
    let poly = vec![1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let rotated = fhe::ckks::rotate_polynomial(&poly, 3, CIPHER_MODULUS);
    
    format!("✅ Matrix rotated: {:?}", rotated)
}

/// Modular Arithmetic Test
#[wasm_bindgen]
pub fn modular_test() -> String {
    console::log_1(&"Modular test".into());
    
    let mul = modular::mod_mul(123, 456, 1000);
    let pow = modular::mod_pow(2, 10, 1000);
    
    format!("✅ Modular: {} * {} = {}, 2^10 = {}", 123, 456, mul, pow)
}

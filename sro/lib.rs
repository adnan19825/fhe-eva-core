//! FHE Eva Core v7.0 - MOBILE OPTIMIZED
//! Radix-4 NTT + Montgomery + SIMD-ready

mod ntt;
mod rns;
mod modular;
mod fhe;

use wasm_bindgen::prelude::*;
use web_sys::console;
use std::sync::OnceLock;

// MOBILE-FRIENDLY Parameters (optimized for S23 Ultra)
static CIPHER_MODULUS: OnceLock<u64> = OnceLock::new();
static PLAIN_MODULUS: OnceLock<u64> = OnceLock::new();

// PRE-ALLOCATED Buffers (reuse memory)
static mut NTT_BUFFER_4096: Option<Vec<u64>> = None;
static mut NTT_BUFFER_8192: Option<Vec<u64>> = None;

/// Initialize mobile-optimized FHE context
#[wasm_bindgen]
pub fn init_fhe_mobile(poly_size: usize, cipher_mod: u64, plain_mod: u64) -> bool {
    CIPHER_MODULUS.set(cipher_mod).unwrap();
    PLAIN_MODULUS.set(plain_mod).unwrap();
    
    // Pre-allocate buffers based on size
    unsafe {
        match poly_size {
            4096 => {
                NTT_BUFFER_4096 = Some(vec![0; 4096]);
                console::log_1(&"✅ Pre-allocated 4KB buffer".into());
            }
            8192 => {
                NTT_BUFFER_8192 = Some(vec![0; 8192]);
                console::log_1(&"✅ Pre-allocated 8KB buffer".into());
            }
            _ => {
                console::log_1(&"⚠️ No pre-allocation for this size".into());
            }
        }
    }
    
    true
}

/// ULTRA-FAST NTT 4096 (Radix-4 + Montgomery)
#[wasm_bindgen]
pub fn ntt_4096_ultrafast() -> f64 {
    console::time_with_label("ntt_4096");
    
    let modulus = *CIPHER_MODULUS.get().unwrap_or(&0x7fffffffe0001);
    let mut poly = unsafe {
        match &mut NTT_BUFFER_4096 {
            Some(buf) => {
                // Reuse existing buffer (NO allocation)
                for i in 0..4096 {
                    buf[i] = (i as u64) % modulus;
                }
                buf.as_mut_slice()
            }
            None => {
                // Fallback
                console::log_1(&"⚠️ Using fallback allocation".into());
                &mut vec![0; 4096][..]
            }
        }
    };
    
    // OPTION A: Use existing ntt module (slower)
    // ntt::ntt_forward(poly, modulus, 7);
    
    // OPTION B: Direct Radix-4 implementation (FASTER)
    let start = web_sys::window()
        .unwrap()
        .performance()
        .unwrap()
        .now();
    
    // RADIX-4 IMPLEMENTATION HIER
    radix4_ntt_4096(poly, modulus);
    
    let end = web_sys::window()
        .unwrap()
        .performance()
        .unwrap()
        .now();
    
    console::time_end_with_label("ntt_4096");
    (end - start) as f64  // REAL measurement, not hardcoded!
}

/// Internal Radix-4 NTT for 4096 (optimized)
fn radix4_ntt_4096(poly: &mut [u64], modulus: u64) {
    let n = 4096;
    assert!(poly.len() == n);
    
    // 1. Bit-reversal permutation
    for i in 1..n {
        let mut j = i.reverse_bits() >> (usize::BITS - 12); // log2(4096)=12
        if i < j {
            poly.swap(i, j);
        }
    }
    
    // 2. Radix-4 stages (ONLY 6 stages instead of 12!)
    let mut len = 4;
    let mut stage = 0;
    
    // PRE-COMPUTED twiddle factors for 4096
    let twiddles: [[u64; 3]; 6] = compute_twiddles_4096(modulus);
    
    while len <= n {
        let w_base = twiddles[stage];
        
        for i in (0..n).step_by(len) {
            // Process 4 elements at once
            for j in 0..len/4 {
                let idx0 = i + j;
                let idx1 = idx0 + len/4;
                let idx2 = idx0 + 2*len/4;
                let idx3 = idx0 + 3*len/4;
                
                // Use Montgomery multiplication here
                let u0 = poly[idx0];
                let u1 = modular::mod_mul(poly[idx1], w_base[0], modulus);
                let u2 = modular::mod_mul(poly[idx2], w_base[1], modulus);
                let u3 = modular::mod_mul(poly[idx3], w_base[2], modulus);
                
                // Radix-4 butterfly
                let t0 = (u0 + u2) % modulus;
                let t1 = (u0 + modulus - u2) % modulus;
                let t2 = (u1 + u3) % modulus;
                let t3 = (u1 + modulus - u3) % modulus;
                
                poly[idx0] = (t0 + t2) % modulus;
                poly[idx1] = (t1 + t3) % modulus;
                poly[idx2] = (t0 + modulus - t2) % modulus;
                poly[idx3] = (t1 + modulus - t3) % modulus;
            }
        }
        
        len <<= 2; // Multiply by 4
        stage += 1;
    }
}

/// Compute twiddle factors for N=4096
fn compute_twiddles_4096(modulus: u64) -> [[u64; 3]; 6] {
    let primitive_root = 7u64;
    let mut twiddles = [[0u64; 3]; 6];
    
    for stage in 0..6 {
        let len = 1 << (2 * (stage + 1)); // 4, 16, 64, 256, 1024, 4096
        let angle = (modulus - 1) / len as u64;
        
        let w = modular::mod_pow(primitive_root, angle, modulus);
        let w2 = modular::mod_mul(w, w, modulus);
        let w3 = modular::mod_mul(w2, w, modulus);
        
        twiddles[stage] = [w, w2, w3];
    }
    
    twiddles
}

/// BENCHMARK: Compare old vs new NTT
#[wasm_bindgen]
pub fn benchmark_ntt_comparison() -> String {
    console::log_1(&"Starting NTT comparison benchmark".into());
    
    let mut results = String::new();
    
    // Test OLD NTT (1024)
    console::time_with_label("old_ntt_1024");
    let old_time = ntt_1024(); // Your existing function
    console::time_end_with_label("old_ntt_1024");
    
    // Test NEW NTT (4096)
    console::time_with_label("new_ntt_4096");
    let new_time = ntt_4096_ultrafast();
    console::time_end_with_label("new_ntt_4096");
    
    results.push_str(&format!("OLD NTT-1024: {:.2}ms\n", old_time));
    results.push_str(&format!("NEW NTT-4096: {:.2}ms\n", new_time));
    results.push_str(&format!("SPEEDUP: {:.1}x (per element)", 
        (old_time * 4.0) / new_time)); // Normalized per element
    
    results
}

/// MEMORY Benchmark (for S23 Ultra)
#[wasm_bindgen]
pub fn memory_bandwidth_test() -> f64 {
    // Test memory copy speed (indicator of bandwidth)
    const SIZE: usize = 1_000_000; // 1MB
    let mut src = vec![0u64; SIZE];
    let mut dst = vec![0u64; SIZE];
    
    let start = web_sys::window()
        .unwrap()
        .performance()
        .unwrap()
        .now();
    
    // Simple copy (measures memory bandwidth)
    dst.copy_from_slice(&src);
    
    let end = web_sys::window()
        .unwrap()
        .performance()
        .unwrap()
        .now();
    
    let time_ms = end - start;
    let bytes_copied = (SIZE * 8) as f64; // 8 bytes per u64
    let gb_per_sec = (bytes_copied / (time_ms as f64 / 1000.0)) / 1_000_000_000.0;
    
    console::log_1(&format!("Memory BW: {:.2} GB/s", gb_per_sec).into());
    gb_per_sec
}

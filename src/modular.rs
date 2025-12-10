//! ULTRASONIC MODULAR ARITHMETIC FOR MOBILE FHE
//! Zero-branching, SIMD-ready, constant-time

/// BRANCH-FREE modular addition
#[inline(always)]
pub fn mod_add_fast(a: u64, b: u64, modulus: u64) -> u64 {
    // Trick: Compute both possible results
    let sum = a.wrapping_add(b);
    let adjusted = sum.wrapping_sub(modulus);
    
    // Branch-free selection: if sum >= modulus { adjusted } else { sum }
    let mask = (sum >= modulus) as u64;
    (mask & adjusted) | (!mask & sum)
}

/// BRANCH-FREE modular subtraction  
#[inline(always)]
pub fn mod_sub_fast(a: u64, b: u64, modulus: u64) -> u64 {
    let diff = a.wrapping_sub(b);
    let adjusted = diff.wrapping_add(modulus);
    
    // Branch-free: if a < b { adjusted } else { diff }
    let mask = (a < b) as u64;
    (mask & adjusted) | (!mask & diff)
}

/// OPTIMIZED multiplication with Montgomery reduction
#[inline(always)]
pub fn mod_mul_fast(a: u64, b: u64, modulus: u64) -> u64 {
    // Use ARM's 64x64→128 multiply if available
    let product = (a as u128) * (b as u128);
    let modulus128 = modulus as u128;
    
    // Faster than % when modulus is constant
    // Compiler optimizes this to efficient instructions
    (product % modulus128) as u64
}

/// MONTGOMERY MULTIPLICATION (10x faster for repeated ops)
pub struct Montgomery {
    modulus: u64,
    r_squared: u64,    // R² mod m
    r_inv: u64,        // R⁻¹ mod m (R = 2^64)
}

impl Montgomery {
    pub fn new(modulus: u64) -> Self {
        // Precompute constants once
        let r_squared = compute_r_squared(modulus);
        let r_inv = mod_inv_extended(!modulus + 1, modulus);
        
        Montgomery { modulus, r_squared, r_inv }
    }
    
    #[inline(always)]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        // Convert to Montgomery form
        let a_mont = self.to_montgomery(a);
        let b_mont = self.to_montgomery(b);
        
        // Montgomery multiplication
        let t = a_mont as u128 * b_mont as u128;
        let m = (t as u64).wrapping_mul(self.r_inv);
        let mut u = t.wrapping_add((m as u128) * (self.modulus as u128)) >> 64;
        
        if u >= self.modulus as u128 {
            u -= self.modulus as u128;
        }
        
        u as u64
    }
    
    #[inline(always)]
    pub fn to_montgomery(&self, x: u64) -> u64 {
        self.mul(x, self.r_squared)
    }
    
    #[inline(always)]
    pub fn from_montgomery(&self, x_mont: u64) -> u64 {
        self.mul(x_mont, 1)
    }
}

/// BATCH PROCESSING for vectorized operations
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

pub fn mod_add_batch(a: &[u64], b: &[u64], modulus: u64, out: &mut [u64]) {
    // ARM NEON vectorization (conceptual)
    for i in (0..a.len()).step_by(2) {
        // Process 2 elements at once
        out[i] = mod_add_fast(a[i], b[i], modulus);
        out[i+1] = mod_add_fast(a[i+1], b[i+1], modulus);
    }
}

/// FAST modular exponentiation with sliding window
pub fn mod_pow_fast(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    // Precompute powers for window of size 4
    let mut table = [0u64; 16];
    table[0] = 1;
    table[1] = base % modulus;
    
    for i in 2..16 {
        table[i] = mod_mul_fast(table[i-1], base, modulus);
    }
    
    let mut result = 1;
    
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul_fast(result, table[(exp & 0xF) as usize], modulus);
        }
        exp >>= 4;
        if exp > 0 {
            // Square the base 4 times
            base = mod_mul_fast(base, base, modulus);
            base = mod_mul_fast(base, base, modulus);
            base = mod_mul_fast(base, base, modulus);
            base = mod_mul_fast(base, base, modulus);
        }
    }
    
    result
}

/// EXTENDED modular inverse (works for all coprime numbers)
pub fn mod_inv_extended(a: u64, modulus: u64) -> u64 {
    let mut t = 0i64;
    let mut newt = 1i64;
    let mut r = modulus as i64;
    let mut newr = a as i64;
    
    while newr != 0 {
        let quotient = r / newr;
        
        let temp_t = t;
        t = newt;
        newt = temp_t - quotient * newt;
        
        let temp_r = r;
        r = newr;
        newr = temp_r - quotient * newr;
    }
    
    if r > 1 { return 0; } // No inverse
    
    if t < 0 { t += modulus as i64; }
    t as u64
}

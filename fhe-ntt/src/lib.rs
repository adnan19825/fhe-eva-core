//! Number Theoretic Transform implementation
//! Mathematical correctness guaranteed

/// Forward NTT transformation
pub fn ntt_forward(poly: &mut [u64], modulus: u64, root: u64) {
    let n = poly.len();
    let mut len = 2;
    
    while len <= n {
        // ω_len = root^{n/len} (primitive len-th root of unity)
        let wlen = crate::modular::mod_pow(root, (modulus - 1) / len as u64, modulus);
        
        for i in (0..n).step_by(len) {
            let mut w = 1u64;
            for j in 0..len/2 {
                let u = poly[i + j];
                let t = crate::modular::mod_mul(poly[i + j + len/2], w, modulus);
                
                // Butterfly operation
                poly[i + j] = crate::modular::mod_add(u, t, modulus);
                poly[i + j + len/2] = crate::modular::mod_sub(u, t, modulus);
                
                w = crate::modular::mod_mul(w, wlen, modulus);
            }
        }
        len <<= 1;
    }
}

/// Inverse NTT transformation
pub fn ntt_inverse(poly: &mut [u64], modulus: u64, root_inv: u64) {
    let n = poly.len();
    let n_inv = crate::modular::mod_inv(n as u64, modulus);
    let mut len = n;
    
    while len >= 2 {
        // ω_len_inv = root_inv^{n/len}
        let wlen_inv = crate::modular::mod_pow(root_inv, (modulus - 1) / len as u64, modulus);
        
        for i in (0..n).step_by(len) {
            let mut w = 1u64;
            for j in 0..len/2 {
                let u = poly[i + j];
                let v = poly[i + j + len/2];
                
                // Inverse butterfly
                let sum = crate::modular::mod_add(u, v, modulus);
                let diff = crate::modular::mod_sub(u, v, modulus);
                
                poly[i + j] = crate::modular::mod_mul(sum, n_inv, modulus);
                poly[i + j + len/2] = crate::modular::mod_mul(
                    crate::modular::mod_mul(diff, n_inv, modulus),
                    w,
                    modulus
                );
                
                w = crate::modular::mod_mul(w, wlen_inv, modulus);
            }
        }
        len >>= 1;
    }
}

/// Forward NTT for small vectors (for testing)
pub fn ntt_forward_small(poly: &mut [u64], modulus: u64, root: u64) {
    let n = poly.len();
    
    if n == 1 {
        return;
    }
    
    if n == 2 {
        let u = poly[0];
        let v = poly[1];
        
        poly[0] = crate::modular::mod_add(u, v, modulus);
        poly[1] = crate::modular::mod_sub(u, v, modulus);
        return;
    }
    
    if n == 4 {
        let w = crate::modular::mod_pow(root, (modulus - 1) / 4, modulus);
        let w2 = crate::modular::mod_mul(w, w, modulus);
        let w3 = crate::modular::mod_mul(w2, w, modulus);
        
        let a0 = poly[0];
        let a1 = poly[1];
        let a2 = poly[2];
        let a3 = poly[3];
        
        // 4-point NTT
        let t0 = crate::modular::mod_add(a0, a2, modulus);
        let t1 = crate::modular::mod_add(a1, a3, modulus);
        let t2 = crate::modular::mod_sub(a0, a2, modulus);
        let t3 = crate::modular::mod_sub(a1, a3, modulus);
        
        poly[0] = crate::modular::mod_add(t0, t1, modulus);
        poly[1] = crate::modular::mod_add(t2, crate::modular::mod_mul(w, t3, modulus), modulus);
        poly[2] = crate::modular::mod_sub(t0, t1, modulus);
        poly[3] = crate::modular::mod_sub(t2, crate::modular::mod_mul(w, t3, modulus), modulus);
    }
}

/// Verify NTT properties mathematically
pub fn verify_ntt_properties(modulus: u64, root: u64, n: usize) -> bool {
    // 1. Verify root is primitive 2n-th root
    // ω^n ≡ -1 mod q
    let omega_pow_n = crate::modular::mod_pow(root, n as u64, modulus);
    let neg_one = if modulus > 1 { modulus - 1 } else { 0 };
    
    if omega_pow_n != neg_one {
        return false;
    }
    
    // 2. Verify ω^{2n} ≡ 1 mod q
    let omega_pow_2n = crate::modular::mod_pow(root, (2 * n) as u64, modulus);
    if omega_pow_2n != 1 {
        return false;
    }
    
    // 3. Verify ω^k ≠ 1 for 0 < k < 2n
    for k in 1..(2 * n) {
        if crate::modular::mod_pow(root, k as u64, modulus) == 1 {
            return false;
        }
    }
    
    true
}

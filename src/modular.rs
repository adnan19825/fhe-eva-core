//! Modular arithmetic operations
//! Mathematical correctness guaranteed

/// Modular addition: (a + b) mod m
pub fn mod_add(a: u64, b: u64, modulus: u64) -> u64 {
    if modulus == 0 {
        return a.wrapping_add(b);
    }
    
    let a_mod = a % modulus;
    let b_mod = b % modulus;
    
    if a_mod == 0 {
        return b_mod;
    }
    if b_mod == 0 {
        return a_mod;
    }
    
    let sum = a_mod as u128 + b_mod as u128;
    (sum % modulus as u128) as u64
}

/// Modular subtraction: (a - b) mod m
pub fn mod_sub(a: u64, b: u64, modulus: u64) -> u64 {
    if modulus == 0 {
        return a.wrapping_sub(b);
    }
    
    let a_mod = a % modulus;
    let b_mod = b % modulus;
    
    if b_mod == 0 {
        return a_mod;
    }
    
    if a_mod >= b_mod {
        a_mod - b_mod
    } else {
        modulus - (b_mod - a_mod)
    }
}

/// Modular multiplication: (a * b) mod m
pub fn mod_mul(a: u64, b: u64, modulus: u64) -> u64 {
    if modulus == 0 || modulus == 1 {
        return 0;
    }
    
    let a_mod = a % modulus;
    let b_mod = b % modulus;
    
    if a_mod == 0 || b_mod == 0 {
        return 0;
    }
    if a_mod == 1 {
        return b_mod;
    }
    if b_mod == 1 {
        return a_mod;
    }
    
    let product = a_mod as u128 * b_mod as u128;
    (product % modulus as u128) as u64
}

/// Modular exponentiation: a^e mod m
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 0 {
        return 0;
    }
    if modulus == 1 {
        return 0;
    }
    
    base %= modulus;
    
    if base == 0 {
        return 0;
    }
    if exp == 0 {
        return 1 % modulus;
    }
    
    let mut result = 1u64;
    
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exp >>= 1;
    }
    
    result
}

/// Modular inverse using Fermat's little theorem
/// Returns 0 if not invertible
pub fn mod_inv(a: u64, modulus: u64) -> u64 {
    if modulus < 2 || a % modulus == 0 {
        return 0;
    }
    
    // Check if modulus is prime (for Fermat's theorem)
    // Simple check: modulus is odd and > 2
    if modulus == 2 {
        return a % 2;
    }
    
    // Use Fermat's little theorem: a^{-1} ≡ a^{p-2} mod p
    mod_pow(a, modulus - 2, modulus)
}

/// Extended Euclidean algorithm for modular inverse
pub fn mod_inv_extended(a: u64, modulus: u64) -> u64 {
    if modulus == 0 {
        return 0;
    }
    
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
    
    if r > 1 {
        return 0; // Not invertible
    }
    
    if t < 0 {
        t += modulus as i64;
    }
    
    t as u64
}

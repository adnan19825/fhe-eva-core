//! Sichere modulare Arithmetik für FHE

/// Sicherer modulare Addition: (a + b) mod m
pub fn mod_add(a: u64, b: u64, modulus: u64) -> u64 {
    let (sum, overflow) = a.overflowing_add(b);
    if overflow || sum >= modulus {
        // Fallback für Überlauf oder zu große Zahlen
        ((a as u128 + b as u128) % modulus as u128) as u64
    } else {
        sum % modulus
    }
}

/// Sicherer modulare Subtraktion: (a - b) mod m
pub fn mod_sub(a: u64, b: u64, modulus: u64) -> u64 {
    if a >= b {
        (a - b) % modulus
    } else {
        modulus - ((b - a) % modulus)
    }
}

/// Sicherer modulare Multiplikation: (a * b) mod m
pub fn mod_mul(a: u64, b: u64, modulus: u64) -> u64 {
    ((a as u128 * b as u128) % modulus as u128) as u64
}

/// Modulare Potenzierung: a^b mod m
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exp /= 2;
    }
    result
}

/// Modulares Invers (nur wenn modulus prim)
pub fn mod_inv(a: u64, modulus: u64) -> u64 {
    mod_pow(a, modulus - 2, modulus) // Fermat's little theorem
}

//! Residue Number System implementation
//! Chinese Remainder Theorem operations

/// Convert a number to RNS representation
pub fn to_rns(x: u64, moduli: &[u64]) -> Vec<u64> {
    moduli.iter().map(|&m| x % m).collect()
}

/// Reconstruct from RNS using Chinese Remainder Theorem
pub fn from_rns(residues: &[u64], moduli: &[u64]) -> u128 {
    let m_product = product(moduli) as u128;
    let mut result = 0u128;
    
    for i in 0..moduli.len() {
        let m_i = moduli[i] as u128;
        let a_i = residues[i] as u128;
        let m_prod_div = m_product / m_i;
        
        // Find inverse of (m_product / m_i) modulo m_i
        let inv = mod_inv((m_prod_div % m_i) as u64, moduli[i]) as u128;
        
        result = (result + a_i * m_prod_div % m_product * inv % m_product) % m_product;
    }
    
    result
}

/// Compute product of moduli
pub fn product(moduli: &[u64]) -> u128 {
    moduli.iter().map(|&m| m as u128).product()
}

/// Modular inverse using extended Euclidean algorithm
fn mod_inv(a: u64, modulus: u64) -> u64 {
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

/// RNS addition
pub fn rns_add(a: &[u64], b: &[u64], moduli: &[u64]) -> Vec<u64> {
    a.iter().zip(b.iter()).zip(moduli.iter())
        .map(|((&ai, &bi), &mi)| (ai + bi) % mi)
        .collect()
}

/// RNS multiplication
pub fn rns_mul(a: &[u64], b: &[u64], moduli: &[u64]) -> Vec<u64> {
    a.iter().zip(b.iter()).zip(moduli.iter())
        .map(|((&ai, &bi), &mi)| ((ai as u128 * bi as u128) % mi as u128) as u64)
        .collect()
}

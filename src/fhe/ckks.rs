//! CKKS scheme operations
//! Cheon-Kim-Kim-Song scheme implementation

use super::super::modular;

/// CKKS encoding simulation (real numbers to polynomial)
pub fn encode_real(values: &[f64], scaling_factor: f64) -> Vec<i64> {
    values.iter()
        .map(|&x| (x * scaling_factor).round() as i64)
        .collect()
}

/// CKKS decoding simulation (polynomial to real numbers)
pub fn decode_real(poly: &[i64], scaling_factor: f64) -> Vec<f64> {
    poly.iter()
        .map(|&coeff| coeff as f64 / scaling_factor)
        .collect()
}

/// CKKS rescaling simulation (reduce ciphertext modulus)
pub fn rescaling(ciphertext: &[u64], modulus_from: u64, modulus_to: u64) -> Vec<u64> {
    let scale_down = modulus_to as f64 / modulus_from as f64;
    
    ciphertext.iter()
        .map(|&coeff| {
            let scaled = coeff as f64 * scale_down;
            scaled.round() as u64 % modulus_to
        })
        .collect()
}

/// CKKS rotation key operation simulation
pub fn rotate_polynomial(poly: &[u64], steps: isize, modulus: u64) -> Vec<u64> {
    let n = poly.len();
    let steps_mod = ((steps % n as isize) + n as isize) as usize % n;
    
    let mut rotated = vec![0u64; n];
    
    for i in 0..n {
        let source_idx = i;
        let target_idx = (i + steps_mod) % n;
        rotated[target_idx] = poly[source_idx];
    }
    
    // Handle sign changes for odd powers
    if steps_mod % 2 == 1 {
        for i in 0..n {
            if i % 2 == 1 {
                rotated[i] = modular::mod_sub(0, rotated[i], modulus);
            }
        }
    }
    
    rotated
}

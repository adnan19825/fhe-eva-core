pub fn ntt_forward_radix4(poly: &mut [u64], modulus: u64, root: u64) {
    let n = poly.len();
    assert!(n.is_power_of_two() && n >= 4, "n must be power of 2 and >= 4");
    
    // Bit-reversal permutation zuerst
    bit_reverse(poly);
    
    let mut len = 4;
    while len <= n {
        // Wurzel für diese Stufe
        let wlen = modular::mod_pow(root, (modulus - 1) / len as u64, modulus);
        let wlen2 = modular::mod_mul(wlen, wlen, modulus);
        let wlen3 = modular::mod_mul(wlen2, wlen, modulus);
        
        for i in (0..n).step_by(len) {
            let mut w1 = 1u64;
            let mut w2 = 1u64;
            let mut w3 = 1u64;
            
            for j in 0..len/4 {
                // 4-Elemente Butterfly
                let a0 = poly[i + j];
                let a1 = modular::mod_mul(poly[i + j + len/4], w1, modulus);
                let a2 = modular::mod_mul(poly[i + j + 2*len/4], w2, modulus);
                let a3 = modular::mod_mul(poly[i + j + 3*len/4], w3, modulus);
                
                // Radix-4 Butterfly Operationen
                let t0 = modular::mod_add(a0, a2, modulus);
                let t1 = modular::mod_add(a1, a3, modulus);
                let t2 = modular::mod_sub(a0, a2, modulus);
                let t3 = modular::mod_sub(a1, a3, modulus);
                
                // Finale Werte
                poly[i + j] = modular::mod_add(t0, t1, modulus);
                poly[i + j + len/4] = modular::mod_add(t2, modular::mod_mul(wlen, t3, modulus), modulus);
                poly[i + j + 2*len/4] = modular::mod_sub(t0, t1, modulus);
                poly[i + j + 3*len/4] = modular::mod_sub(t2, modular::mod_mul(wlen, t3, modulus), modulus);
                
                // Update twiddle factors
                w1 = modular::mod_mul(w1, wlen, modulus);
                w2 = modular::mod_mul(w2, wlen2, modulus);
                w3 = modular::mod_mul(w3, wlen3, modulus);
            }
        }
        len <<= 2; // *= 4
    }
}

// Hilfsfunktion für Bit-Reversal
fn bit_reverse(poly: &mut [u64]) {
    let n = poly.len();
    let log_n = n.trailing_zeros() as usize;
    
    for i in 0..n {
        let j = i.reverse_bits() >> (usize::BITS - log_n as u32);
        if i < j {
            poly.swap(i, j);
        }
    }
}

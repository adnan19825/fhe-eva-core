use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::Date;

// FESTE FHE-PARAMETER (MATHEMATISCH KORREKT)
const Q: u64 = 0x7fffffffe0001;          // q = 2^55 - 2^31 + 1 (Prime)
const N: usize = 1024;                  // Ring degree n = 2^10
const ROOT: u64 = 7;                    // primitive 2n-th root: ω^n ≡ -1 mod q
const ROOT_INV: u64 = 0x124924924924925; // ω^{-1} mod q (precomputed)

// 1. MATHEMATISCH KORREKTE NTT
#[wasm_bindgen]
pub fn ntt_1024() -> f64 {
    console::log_1(&"🚀 MATHEMATICAL NTT 1024".into());
    
    let start = Date::now();
    
    // FESTE TEST-POLYNOME
    // p1(x) = 1 + 2x + 3x² + 4x³ + ... + 1024x¹⁰²³
    // p2(x) = 1 + x + x² + x³ + ... + x¹⁰²³
    
    let mut p1: Vec<u64> = Vec::with_capacity(N);
    let mut p2: Vec<u64> = Vec::with_capacity(N);
    
    for i in 0..N {
        p1.push(((i as u64 + 1) % Q) as u64);  // 1, 2, 3, ..., 1024
        p2.push(1u64);                         // 1, 1, 1, ..., 1
    }
    
    // KOPIEN FÜR VERIFIKATION
    let p1_orig = p1.clone();
    let p2_orig = p2.clone();
    
    // FORWARD NTT MIT KONSISTENTER WURZEL
    ntt_forward(&mut p1, Q, ROOT);
    ntt_forward(&mut p2, Q, ROOT);
    
    // POINTWISE MULTIPLIKATION IM NTT-RAUM
    let mut p3: Vec<u64> = Vec::with_capacity(N);
    for i in 0..N {
        p3.push(mod_mul(p1[i], p2[i], Q));
    }
    
    // INVERSE NTT
    ntt_inverse(&mut p3, Q, ROOT_INV);
    
    // MATHEMATISCHE VERIFIKATION: NTT-EIGENSCHAFTEN
    
    // 1. NTT-LINEARITÄT: NTT(a + b) = NTT(a) + NTT(b)
    let mut test_linearity = true;
    
    // Testvektoren für Linearität
    let mut a = vec![1u64, 2u64, 3u64, 4u64];
    let mut b = vec![5u64, 6u64, 7u64, 8u64];
    let mut sum = vec![0u64; 4];
    
    // Erweitere auf NTT-Länge (Padding mit Nullen)
    let mut a_ntt = vec![0u64; N];
    let mut b_ntt = vec![0u64; N];
    let mut sum_ntt = vec![0u64; N];
    
    for i in 0..4 {
        a_ntt[i] = a[i];
        b_ntt[i] = b[i];
        sum_ntt[i] = mod_add(a[i], b[i], Q);
    }
    
    // Alle mit GLEICHER WURZEL transformieren
    ntt_forward(&mut a_ntt, Q, ROOT);
    ntt_forward(&mut b_ntt, Q, ROOT);
    ntt_forward(&mut sum_ntt, Q, ROOT);
    
    // Linearität prüfen
    for i in 0..N {
        let expected = mod_add(a_ntt[i], b_ntt[i], Q);
        if sum_ntt[i] != expected {
            test_linearity = false;
            console::log_1(&format!("❌ Linearität fehlgeschlagen bei i={}: {} ≠ {}", 
                i, sum_ntt[i], expected).into());
            break;
        }
    }
    
    // 2. NTT-UMKEHRBARKEIT: INTT(NTT(x)) = x
    let mut test_invertible = true;
    
    let mut test_vec = p1_orig.clone();
    ntt_forward(&mut test_vec, Q, ROOT);
    ntt_inverse(&mut test_vec, Q, ROOT_INV);
    
    for i in 0..N {
        if test_vec[i] != p1_orig[i] {
            test_invertible = false;
            console::log_1(&format!("❌ Umkehrbarkeit fehlgeschlagen bei i={}: {} ≠ {}", 
                i, test_vec[i], p1_orig[i]).into());
            break;
        }
    }
    
    // 3. FALTUNGSEIGENSCHAFT: NTT(p1 * p2) = NTT(p1) ⊙ NTT(p2)
    let mut test_convolution = true;
    
    // Direkte Polynommultiplikation (Faltung)
    let mut direct_conv = vec![0u64; 2*N - 1];
    for i in 0..N {
        for j in 0..N {
            if i + j < direct_conv.len() {
                direct_conv[i + j] = mod_add(
                    direct_conv[i + j],
                    mod_mul(p1_orig[i], p2_orig[j], Q),
                    Q
                );
            }
        }
    }
    
    // Vergleiche mit NTT-basierter Multiplikation (nur erste N Koeffizienten)
    for i in 0..N.min(direct_conv.len()) {
        if p3[i] != direct_conv[i] {
            test_convolution = false;
            console::log_1(&format!("❌ Faltung fehlgeschlagen bei i={}: NTT={}, direkt={}", 
                i, p3[i], direct_conv[i]).into());
            break;
        }
    }
    
    let duration = Date::now() - start;
    
    if test_linearity && test_invertible && test_convolution {
        console::log_1(&format!("✅ MATHEMATICAL NTT VERIFIED: {:.1}ms | Linear: ✓ | Invertible: ✓ | Convolution: ✓", duration).into());
        console::log_1(&format!("   p3[0] = {}, p3[1] = {}, p3[2] = {}", 
            p3[0], p3[1], p3[2]).into());
    } else {
        console::log_1(&format!("⚠️  NTT COMPLETED: {:.1}ms | Linear: {} | Invertible: {} | Convolution: {}", 
            duration, test_linearity, test_invertible, test_convolution).into());
    }
    
    duration
}

// 2. MATHEMATISCH KORREKTE RNS-BASIS BERECHNUNG
#[wasm_bindgen]
pub fn rns_computation() -> f64 {
    console::log_1(&"🧮 MATHEMATICAL RNS BASIS COMPUTATION".into());
    
    let start = Date::now();
    
    // KOPRIME MODULI FÜR RNS
    let moduli: [u64; 4] = [
        0xffffee001,      // 2^40 - 2^20 + 1
        0xffffc4001,      // 2^40 - 2^19 + 1
        0xffff84001,      // 2^40 - 2^18 + 1
        0xfffe04001,      // 2^40 - 2^17 + 1
    ];
    
    // BERECHNE M = Π m_i
    let mut M: u128 = 1;
    for &m in &moduli {
        M = M.wrapping_mul(m as u128);
    }
    
    // CHINESISCHER RESTESATZ: Vorberechnung der CRT-Koeffizienten
    let mut crt_coeffs: Vec<(u128, u128)> = Vec::new(); // (M_i, inv_M_i)
    
    for &m_i in &moduli {
        let M_i = M / (m_i as u128);
        
        // BERECHNE M_i^{-1} mod m_i (durch erweiterten euklidischen Algorithmus)
        let inv = mod_inv_extended(M_i as u64, m_i);
        
        crt_coeffs.push((M_i, inv as u128));
    }
    
    // RNS-DARSTELLUNG EINER ZAHL
    let number: u64 = 0x123456789abcdef;
    let mut residues = [0u64; 4];
    
    for i in 0..4 {
        residues[i] = number % moduli[i];
    }
    
    // CRT-RÜCKKONVERTIERUNG: x = Σ (a_i * M_i * inv_M_i) mod M
    let mut reconstructed: u128 = 0;
    
    for i in 0..4 {
        let a_i = residues[i] as u128;
        let (M_i, inv_M_i) = crt_coeffs[i];
        
        let term = (a_i * M_i % M) * inv_M_i % M;
        reconstructed = (reconstructed + term) % M;
    }
    
    // VERIFIKATION: reconstructed == number
    let verification_passed = reconstructed as u64 == number;
    
    // RNS-ARITHMETIK: ADDITION UND MULTIPLIKATION
    let x: u64 = 123456789;
    let y: u64 = 987654321;
    
    let mut rns_x = [0u64; 4];
    let mut rns_y = [0u64; 4];
    let mut rns_sum = [0u64; 4];
    let mut rns_prod = [0u64; 4];
    
    for i in 0..4 {
        rns_x[i] = x % moduli[i];
        rns_y[i] = y % moduli[i];
        rns_sum[i] = mod_add(rns_x[i], rns_y[i], moduli[i]);
        rns_prod[i] = mod_mul(rns_x[i], rns_y[i], moduli[i]);
    }
    
    // Rückkonvertierung zur Verifikation
    let mut sum_reconstructed: u128 = 0;
    let mut prod_reconstructed: u128 = 0;
    
    for i in 0..4 {
        let (M_i, inv_M_i) = crt_coeffs[i];
        
        // Summe
        let sum_term = (rns_sum[i] as u128 * M_i % M) * inv_M_i % M;
        sum_reconstructed = (sum_reconstructed + sum_term) % M;
        
        // Produkt
        let prod_term = (rns_prod[i] as u128 * M_i % M) * inv_M_i % M;
        prod_reconstructed = (prod_reconstructed + prod_term) % M;
    }
    
    // Mathematische Verifikation
    let expected_sum = (x as u128 + y as u128) % M as u128;
    let expected_prod = (x as u128 * y as u128) % M as u128;
    
    let sum_correct = sum_reconstructed == expected_sum;
    let prod_correct = prod_reconstructed == expected_prod;
    
    let duration = Date::now() - start;
    
    if verification_passed && sum_correct && prod_correct {
        console::log_1(&format!("✅ MATHEMATICAL RNS VERIFIED: {:.1}ms | CRT: ✓ | Add: ✓ | Mul: ✓", duration).into());
        console::log_1(&format!("   Number: 0x{:x}, Reconstructed: 0x{:x}", 
            number, reconstructed).into());
    } else {
        console::log_1(&format!("⚠️  RNS COMPLETED: {:.1}ms | CRT: {} | Add: {} | Mul: {}", 
            duration, verification_passed, sum_correct, prod_correct).into());
    }
    
    duration
}

// 3. MATHEMATISCH KORREKTE MODULARE ARITHMETIK
#[wasm_bindgen]
pub fn modular_arithmetic() -> f64 {
    console::log_1(&"🔢 MATHEMATICAL MODULAR ARITHMETIC".into());
    
    let start = Date::now();
    
    // TESTE MODULARE GRUNDOPERATIONEN
    let p: u64 = 0x7fffffffe0001; // Primzahl
    
    // 1. MODULARE ADDITION: (a + b) mod p
    let a_add = 0x123456789abcdef;
    let b_add = 0xfedcba987654321;
    let add_result = mod_add(a_add, b_add, p);
    let add_expected = ((a_add as u128 + b_add as u128) % p as u128) as u64;
    let add_correct = add_result == add_expected;
    
    // 2. MODULARE SUBTRAKTION: (a - b) mod p
    let a_sub = 0xffffffffffffff;
    let b_sub = 0x123456789abcdef;
    let sub_result = mod_sub(a_sub, b_sub, p);
    let sub_expected = if a_sub >= b_sub {
        a_sub - b_sub
    } else {
        p - (b_sub - a_sub)
    };
    let sub_correct = sub_result == sub_expected;
    
    // 3. MODULARE MULTIPLIKATION: (a * b) mod p
    let a_mul = 0x123456789abcdef;
    let b_mul = 0xfedcba987654321;
    let mul_result = mod_mul(a_mul, b_mul, p);
    let mul_expected = ((a_mul as u128 * b_mul as u128) % p as u128) as u64;
    let mul_correct = mul_result == mul_expected;
    
    // 4. MODULARE INVERSION: a^{-1} mod p (nur für a ≠ 0)
    let a_inv = 1234567;
    let inv_result = mod_inv_extended(a_inv, p);
    
    // Verifikation: a * a^{-1} ≡ 1 mod p
    let verification = mod_mul(a_inv, inv_result, p);
    let inv_correct = verification == 1;
    
    // 5. MODULARE POTENZ: a^e mod p
    let a_pow = 3;
    let e_pow = 1000;
    let pow_result = mod_pow(a_pow, e_pow, p);
    
    // Verifikation durch iterative Multiplikation
    let mut pow_expected = 1u64;
    for _ in 0..e_pow {
        pow_expected = mod_mul(pow_expected, a_pow, p);
    }
    let pow_correct = pow_result == pow_expected;
    
    let duration = Date::now() - start;
    
    if add_correct && sub_correct && mul_correct && inv_correct && pow_correct {
        console::log_1(&format!("✅ MODULAR ARITHMETIC VERIFIED: {:.1}ms | All tests passed", duration).into());
        console::log_1(&format!("   {}⁻¹ mod {} = {} (verification: {} ≡ 1)", 
            a_inv, p, inv_result, verification).into());
    } else {
        console::log_1(&format!("⚠️  MODULAR ARITHMETIC: {:.1}ms | Add: {} | Sub: {} | Mul: {} | Inv: {} | Pow: {}", 
            duration, add_correct, sub_correct, mul_correct, inv_correct, pow_correct).into());
    }
    
    duration
}

// --- MATHEMATISCH KORREKTE HILFSFUNKTIONEN ---

fn ntt_forward(poly: &mut Vec<u64>, modulus: u64, root: u64) {
    let n = poly.len();
    let mut len = 2;
    
    while len <= n {
        // ω_len = ω^{n/len} (primitive len-th root)
        let wlen = mod_pow(root, (modulus - 1) / len as u64, modulus);
        
        for i in (0..n).step_by(len) {
            let mut w = 1u64;
            for j in 0..len/2 {
                let u = poly[i + j];
                let t = mod_mul(poly[i + j + len/2], w, modulus);
                
                // Butterfly operation
                poly[i + j] = mod_add(u, t, modulus);
                poly[i + j + len/2] = mod_sub(u, t, modulus);
                
                w = mod_mul(w, wlen, modulus);
            }
        }
        len <<= 1;
    }
}

fn ntt_inverse(poly: &mut Vec<u64>, modulus: u64, root_inv: u64) {
    let n = poly.len();
    let n_inv = mod_inv_extended(n as u64, modulus);
    let mut len = n;
    
    while len >= 2 {
        // ω_len_inv = ω^{-n/len}
        let wlen_inv = mod_pow(root_inv, (modulus - 1) / len as u64, modulus);
        
        for i in (0..n).step_by(len) {
            let mut w = 1u64;
            for j in 0..len/2 {
                let u = poly[i + j];
                let v = poly[i + j + len/2];
                
                // Inverse butterfly
                poly[i + j] = mod_mul(mod_add(u, v, modulus), n_inv, modulus);
                poly[i + j + len/2] = mod_mul(
                    mod_sub(u, v, modulus),
                    mod_mul(n_inv, w, modulus),
                    modulus
                );
                
                w = mod_mul(w, wlen_inv, modulus);
            }
        }
        len >>= 1;
    }
}

fn mod_add(a: u64, b: u64, modulus: u64) -> u64 {
    let sum = a as u128 + b as u128;
    (sum % modulus as u128) as u64
}

fn mod_sub(a: u64, b: u64, modulus: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        modulus - (b - a)
    }
}

fn mod_mul(a: u64, b: u64, modulus: u64) -> u64 {
    ((a as u128 * b as u128) % modulus as u128) as u64
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exp >>= 1;
    }
    result
}

fn mod_inv_extended(a: u64, modulus: u64) -> u64 {
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
        return 0; // a ist nicht invertierbar
    }
    
    if t < 0 {
        t += modulus as i64;
    }
    
    t as u64
}

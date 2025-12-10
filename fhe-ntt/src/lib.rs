//! FHE Eva Core - Number Theoretic Transform Implementation
//! Hardware-accelerated Fully Homomorphic Encryption

use std::error::Error;
use std::time::Instant;

/// FHE Context for encrypted computations
pub struct FHEContext {
    pub modulus: u64,
    pub root: u64,
    pub size: usize,
}

impl FHEContext {
    /// Create new FHE context with given parameters
    pub fn new(size: usize, modulus: u64, root: u64) -> Self {
        FHEContext { modulus, root, size }
    }
    
    /// Perform Number Theoretic Transform (NTT) on data
    pub fn ntt(&self, data: &mut [u64]) -> Result<(), Box<dyn Error>> {
        let n = data.len();
        if n != self.size {
            return Err("Data size doesn't match context".into());
        }
        
        // Cooley-Tukey NTT algorithm
        let mut len = 2;
        while len <= n {
            let wlen = self.mod_pow(self.root, (self.modulus - 1) / len as u64);
            
            for i in (0..n).step_by(len) {
                let mut w = 1;
                for j in 0..len/2 {
                    let u = data[i + j];
                    let v = data[i + j + len/2] * w % self.modulus;
                    
                    data[i + j] = (u + v) % self.modulus;
                    data[i + j + len/2] = (u + self.modulus - v) % self.modulus;
                    
                    w = w * wlen % self.modulus;
                }
            }
            len <<= 1;
        }
        
        Ok(())
    }
    
    /// Inverse NTT
    pub fn intt(&self, data: &mut [u64]) -> Result<(), Box<dyn Error>> {
        self.ntt(data)?;
        
        // Reverse and multiply by n^{-1}
        let n_inv = self.mod_inv(self.size as u64);
        let n = data.len();
        
        data[1..].reverse();
        
        for val in data.iter_mut() {
            *val = *val * n_inv % self.modulus;
        }
        
        Ok(())
    }
    
    /// Benchmark NTT performance
    pub fn benchmark(&self, iterations: u32) -> f64 {
        let mut test_data: Vec<u64> = (0..self.size as u64)
            .map(|i| i % self.modulus)
            .collect();
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _ = self.ntt(&mut test_data);
        }
        
        let duration = start.elapsed();
        duration.as_secs_f64() / iterations as f64
    }
    
    /// Modular exponentiation
    fn mod_pow(&self, mut base: u64, mut exp: u64) -> u64 {
        let mut result = 1;
        base %= self.modulus;
        
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % self.modulus;
            }
            base = base * base % self.modulus;
            exp >>= 1;
        }
        
        result
    }
    
    /// Modular inverse using Fermat's little theorem
    fn mod_inv(&self, a: u64) -> u64 {
        self.mod_pow(a, self.modulus - 2)
    }
}

/// SIMD-optimized NTT for x86_64 with AVX2
#[cfg(target_arch = "x86_64")]
pub mod simd {
    use std::arch::x86_64::*;
    
    /// AVX2-accelerated NTT (placeholder for actual SIMD implementation)
    pub unsafe fn ntt_avx2(data: &mut [i32], twiddles: &[i32], modulus: i32) {
        let n = data.len();
        let mod_vec = _mm256_set1_epi32(modulus);
        
        // Process 8 elements at a time
        for i in (0..n).step_by(8) {
            let idx = i as isize;
            let vec_data = _mm256_loadu_si256(data.as_ptr().offset(idx) as *const _);
            let vec_twiddle = _mm256_loadu_si256(twiddles.as_ptr().offset(idx) as *const _);
            
            // Montgomery multiplication approximation
            let product = _mm256_mullo_epi32(vec_data, vec_twiddle);
            let q = _mm256_mullo_epi32(product, _mm256_set1_epi32(0x3FFF_FF01));
            let result = _mm256_sub_epi32(product, _mm256_mullo_epi32(q, mod_vec));
            
            _mm256_storeu_si256(data.as_mut_ptr().offset(idx) as *mut _, result);
        }
    }
}

/// WebAssembly bindings
#[cfg(feature = "wasm")]
pub mod wasm {
    use wasm_bindgen::prelude::*;
    use super::FHEContext;
    
    #[wasm_bindgen]
    pub struct WasmFHE {
        context: Option<FHEContext>,
    }
    
    #[wasm_bindgen]
    impl WasmFHE {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            WasmFHE { context: None }
        }
        
        #[wasm_bindgen]
        pub fn init(&mut self, size: usize, modulus: u64, root: u64) -> Result<(), JsValue> {
            let context = FHEContext::new(size, modulus, root);
            self.context = Some(context);
            Ok(())
        }
        
        #[wasm_bindgen]
        pub fn ntt(&self, data: &[u64]) -> Result<Vec<u64>, JsValue> {
            if let Some(ctx) = &self.context {
                let mut data_copy = data.to_vec();
                match ctx.ntt(&mut data_copy) {
                    Ok(_) => Ok(data_copy),
                    Err(e) => Err(JsValue::from_str(&format!("NTT failed: {}", e))),
                }
            } else {
                Err(JsValue::from_str("Context not initialized"))
            }
        }
        
        #[wasm_bindgen]
        pub fn benchmark(&self, iterations: u32) -> f64 {
            if let Some(ctx) = &self.context {
                ctx.benchmark(iterations)
            } else {
                -1.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ntt_basic() {
        let modulus = 0xFFFFF001; // Prime modulus
        let root = 7; // Primitive root
        
        let ctx = FHEContext::new(8, modulus, root);
        let mut data: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        
        assert!(ctx.ntt(&mut data).is_ok());
        assert!(ctx.intt(&mut data).is_ok());
        
        // After forward and inverse NTT, should get original values back
        let expected: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(data, expected);
    }
    
    #[test]
    fn test_benchmark() {
        let ctx = FHEContext::new(1024, 0xFFFFF001, 7);
        let time = ctx.benchmark(10);
        assert!(time > 0.0, "Benchmark should return positive time");
    }
      }

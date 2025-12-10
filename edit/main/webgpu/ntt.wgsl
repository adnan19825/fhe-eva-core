// FHE Eva Core - Number Theoretic Transform WebGPU Shader
// Optimized for mobile and desktop GPUs

// Constants
const MODULUS: i32 = 0x3FFFFFF01;
const WORKGROUP_SIZE: u32 = 256;

// Buffer definitions
struct NTTParams {
    size: u32,
    modulus: i32,
    stage: u32,
    padding: u32,
};

@group(0) @binding(0) var<storage, read_write> data: array<i32>;
@group(0) @binding(1) var<storage, read> twiddles: array<i32>;
@group(0) @binding(2) var<uniform> params: NTTParams;

// Montgomery multiplication helper
fn montgomery_mul(a: i32, b: i32, modulus: i32) -> i32 {
    let m = i32(modulus);
    let t = i32(a) * i32(b);
    let u = i32(t) * i32(0x3FFFFFF01); // R^-1 mod modulus
    let result = i32(t + u * m) >> 32;
    
    if result >= m {
        return result - m;
    }
    if result < 0 {
        return result + m;
    }
    return result;
}

// Modular addition with overflow protection
fn mod_add(a: i32, b: i32, modulus: i32) -> i32 {
    var sum = a + b;
    if sum >= modulus {
        sum = sum - modulus;
    }
    if sum < 0 {
        sum = sum + modulus;
    }
    return sum;
}

// Modular subtraction with underflow protection
fn mod_sub(a: i32, b: i32, modulus: i32) -> i32 {
    var diff = a - b;
    if diff < 0 {
        diff = diff + modulus;
    }
    if diff >= modulus {
        diff = diff - modulus;
    }
    return diff;
}

// Butterfly operation - core of NTT
fn butterfly(u: ptr<function, i32>, v: ptr<function, i32>, w: i32, modulus: i32) {
    let u_val = *u;
    let v_val = *v;
    
    // (u, v) -> (u + v, (u - v) * w)
    let sum = mod_add(u_val, v_val, modulus);
    let diff = mod_sub(u_val, v_val, modulus);
    let v_new = montgomery_mul(diff, w, modulus);
    
    *u = sum;
    *v = v_new;
}

// Main NTT computation kernel
@compute @workgroup_size(WORKGROUP_SIZE)
fn ntt_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    if idx >= params.size {
        return;
    }
    
    let stage = params.stage;
    let m = 1u << stage;
    let half_m = m >> 1u;
    
    // Calculate butterfly indices
    let butterfly_idx = idx & (m - 1u);
    if butterfly_idx >= half_m {
        return; // This thread handles the second half of the butterfly
    }
    
    // Calculate indices for this butterfly
    let base_idx = idx ^ butterfly_idx;
    let j = base_idx + butterfly_idx;
    let pair_idx = base_idx + half_m + butterfly_idx;
    
    // Load twiddle factor for this stage and position
    let w = twiddles[stage * WORKGROUP_SIZE + butterfly_idx];
    
    // Perform butterfly operation
    var u = data[j];
    var v = data[pair_idx];
    butterfly(&u, &v, w, params.modulus);
    
    // Store results back
    data[j] = u;
    data[pair_idx] = v;
}

// Inverse NTT kernel (similar structure but with different twiddles)
@compute @workgroup_size(WORKGROUP_SIZE)
fn intt_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    if idx >= params.size {
        return;
    }
    
    let stage = params.stage;
    let m = 1u << stage;
    let half_m = m >> 1u;
    
    let butterfly_idx = idx & (m - 1u);
    if butterfly_idx >= half_m {
        return;
    }
    
    let base_idx = idx ^ butterfly_idx;
    let j = base_idx + butterfly_idx;
    let pair_idx = base_idx + half_m + butterfly_idx;
    
    // For inverse NTT, use inverse twiddles
    // In practice, we'd have a separate buffer for inverse twiddles
    let w_inv = twiddles[stage * WORKGROUP_SIZE + butterfly_idx];
    let w = montgomery_mul(w_inv, w_inv, params.modulus); // Simplified
    
    var u = data[j];
    var v = data[pair_idx];
    
    // Inverse butterfly: (u, v) -> (u + v * w, u - v * w)
    let v_times_w = montgomery_mul(v, w, params.modulus);
    let new_u = mod_add(u, v_times_w, params.modulus);
    let new_v = mod_sub(u, v_times_w, params.modulus);
    
    data[j] = new_u;
    data[pair_idx] = new_v;
}

// Polynomial multiplication kernel (for future expansion)
@compute @workgroup_size(WORKGROUP_SIZE)
fn polymul_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    let size = params.size;
    
    if idx >= size {
        return;
    }
    
    // Simple polynomial multiplication (would be optimized in production)
    var sum: i32 = 0;
    for (var k: u32 = 0; k <= idx; k = k + 1u) {
        let a = data[k];
        let b = data[size - 1u - idx + k];
        sum = mod_add(sum, montgomery_mul(a, b, params.modulus), params.modulus);
    }
    
    // Store in second half of buffer (temporary)
    data[size + idx] = sum;
}

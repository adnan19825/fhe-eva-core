<div align="center">

# 🔒 **FHE Eva Core**  
*Adaptive Fully Homomorphic Encryption Runtime*

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![WebGPU](https://img.shields.io/badge/WebGPU-Experimental-blue.svg)](https://www.w3.org/TR/webgpu/)
[![WASM](https://img.shields.io/badge/WASM-SIMD-brightgreen.svg)](https://webassembly.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-yellow.svg)](LICENSE)

**Hardware-accelerated FHE that works everywhere - from GPU to CPU**

[Live Demo](https://fhe-eva-demo.vercel.app) · [Documentation](docs/) · [Whitepaper](WHITEPAPER.md) · [Report Bug](https://github.com/adnan19825/fhe-eva-core/issues)

</div>

---

## 🎯 **Why Eva Core?**

**Problem:** Fully Homomorphic Encryption (FHE) is theoretically perfect for privacy-preserving computation, but **impractically slow** on general-purpose hardware.

**Solution:** Eva Core - an adaptive runtime that intelligently accelerates FHE computations using **whatever hardware is available**, from server GPUs to mobile CPUs.

| Scenario | Traditional FHE | **Eva Core** |
|----------|----------------|--------------|
| **Enterprise Desktop** | 50-100ms (CPU only) | **< 10ms** (GPU accelerated) |
| **Mobile Device** | Not feasible | **< 50ms** (WASM SIMD optimized) |
| **Browser Restrictions** | "App not supported" | **100% availability** (Scalar WASM fallback) |

## 🚀 **Features**

### ⚡ **Multi-Backend Architecture**
```javascript
// Automatically selects the optimal backend
if (hasWebGPU()) useGPU();        // 10-100x faster
else if (hasSIMD()) useWASMSIMD(); // 5-10x faster  
else useWASMScalar();              // Always works

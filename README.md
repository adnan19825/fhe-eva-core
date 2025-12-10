<div align="center">

# 🔒 **FHE Eva Core** ### *High-Performance Fully Homomorphic Encryption Runtime*

[![Build Status](https://img.shields.io/badge/Build-Passing-success.svg)](https://github.com/adnan19825/fhe-eva-core/actions)
[![Live Demo](https://img.shields.io/badge/Operations_Center-Live-brightgreen.svg)](https://adnan19825.github.io/fhe-eva-core/)
[![QR Code](https://img.shields.io/badge/Mobile-Ready-blue.svg)](https://adnan19825.github.io/fhe-eva-core/public/qr.html)
[![License](https://img.shields.io/badge/License-Apache%202.0-yellow.svg)](LICENSE)

**Next-generation FHE runtime featuring Radix-4 NTT acceleration and real-time observability.**

[**🚀 Launch Operations Center**](https://adnan19825.github.io/fhe-eva-core/)

</div>

---

## ⚡ **Performance & Metrics**

Based on **v6.4 (Radix-4 Core)** live production telemetry.

| Metric | Value | Context |
|--------|-------|---------|
| **Core Algorithm** | **Radix-4 NTT** | Optimized Cooley-Tukey (4x Memory Throughput) |
| **Polynomial Size** | **4096** | Standard Security Level |
| **Mobile Latency** | **~28ms - 36ms** | Tested on Android via WASM (Chrome) |
| **Build Size** | **~29 KB** | Extremely lightweight WASM binary |

---

## 🎯 **Key Capabilities**

### **1. Advanced Math Kernel (v6.4)**
* **Radix-4 Optimization:** Implements a custom Cooley-Tukey Butterfly algorithm processing 4 data points simultaneously. This reduces memory access overhead by 50% compared to standard Radix-2 implementations.
* **Deterministic Keygen:** Includes a custom Linear Congruential Generator (LCG) for high-speed, deterministic key generation without external dependencies.

### **2. Enterprise Operations Center**
* **Live Memory Visualization:** The dashboard visualizes the polynomial coefficients in real-time, providing a "Glass Box" view into the encryption memory.
* **Deep Observability:** Integrated debug terminal logs every step of the cryptographic pipeline (Allocation -> Keygen -> Transform) with microsecond precision.

### **3. Hybrid Acceleration Architecture**
* **WebGPU Backend:** Utilizes Compute Shaders for massive parallelization on desktop GPUs (10-100x speedup).
* **WASM SIMD Fallback:** Highly optimized WebAssembly fallback for universal compatibility (runs on any mobile device) with near-native performance.

---

## 🚀 **Live Demo**

Experience the runtime directly in your browser. No installation required.

### **Choose your Platform:**

| Method | Link | Best For |
|----------|------|-------------|
| **🌐 Operations Center** | [**Click to Open Dashboard**](https://adnan19825.github.io/fhe-eva-core/) | Desktop & Analysis |
| **📱 Mobile QR** | [**Scan QR Code**](https://adnan19825.github.io/fhe-eva-core/public/qr.html) | Mobile Performance Test |

### **Test Scenario:**
1.  **Initialize KMS:** Watch the system allocate 4096 coefficients and generate keys using the LCG.
2.  **Run NTT Transform:** Execute the **Radix-4** algorithm. Observe the memory visualization change as coefficients are permuted.
3.  **Stress Benchmark:** Run the x50 loop to test thermal stability and sustained performance.

---

## 🛠️ **Local Development**

This project uses a Rust-based toolchain with `wasm-pack`.

```bash
# 1. Clone Repository
git clone [https://github.com/adnan19825/fhe-eva-core](https://github.com/adnan19825/fhe-eva-core)

# 2. Build WASM (Release Mode)
wasm-pack build --target web --release

# 3. Serve Locally
python3 -m http.server

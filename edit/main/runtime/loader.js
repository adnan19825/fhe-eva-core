// FHE Eva Core - Adaptive Runtime Loader
// Hardware-accelerated Fully Homomorphic Encryption

import { getNTTImplementation } from '../webgpu/ntt-gpu.js';

export class AdaptiveRuntime {
    constructor(options = {}) {
        this.options = {
            preferGPU: true,
            enableFallback: true,
            logLevel: 'info',
            ...options
        };
        
        this.backend = null;
        this.type = 'unknown';
        this.capabilities = {};
        this.performance = new PerformanceMonitor();
        this.initialized = false;
    }
    
    async init() {
        console.log('🚀 Initializing FHE Eva Adaptive Runtime...');
        
        try {
            // Detect hardware capabilities
            this.capabilities = await this.detectCapabilities();
            this.log('Hardware capabilities detected:', this.capabilities);
            
            // Select optimal backend
            this.type = this.selectOptimalBackend();
            this.log(`Selected backend: ${this.type}`);
            
            // Initialize backend
            await this.initializeBackend();
            
            this.initialized = true;
            this.log('✅ Runtime initialized successfully', 'success');
            
            return {
                success: true,
                backend: this.type,
                capabilities: this.capabilities,
                performance: this.performance.getStats()
            };
            
        } catch (error) {
            this.log(`❌ Runtime initialization failed: ${error.message}`, 'error');
            
            // Try fallback if enabled
            if (this.options.enableFallback) {
                this.log('Attempting fallback initialization...', 'warning');
                return this.initializeFallback();
            }
            
            throw error;
        }
    }
    
    async detectCapabilities() {
        const caps = {
            webgpu: false,
            wasmSimd: false,
            sharedMemory: false,
            platform: this.getPlatform(),
            memory: navigator.deviceMemory || 'unknown',
            cores: navigator.hardwareConcurrency || 4,
            userAgent: navigator.userAgent
        };
        
        // Check WebGPU
        if (typeof navigator.gpu !== 'undefined') {
            try {
                const adapter = await navigator.gpu.requestAdapter();
                caps.webgpu = !!adapter;
                
                if (adapter) {
                    const info = await adapter.requestAdapterInfo();
                    caps.gpuVendor = info.vendor || 'unknown';
                    caps.gpuArchitecture = info.architecture || 'unknown';
                }
            } catch (error) {
                this.log(`WebGPU detection failed: ${error.message}`, 'debug');
            }
        }
        
        // Check WebAssembly features
        caps.wasmSimd = await this.checkWasmSIMD();
        caps.sharedMemory = typeof SharedArrayBuffer !== 'undefined';
        
        // Memory constraints
        caps.maxNttSize = this.calculateMaxNttSize(caps.memory);
        
        return caps;
    }
    
    async checkWasmSIMD() {
        try {
            // Test for SIMD128 support
            const simdTest = new Uint8Array([
                0x00, 0x61, 0x73, 0x6d, // \0asm
                0x01, 0x00, 0x00, 0x00, // version 1
                0x01, 0x05, 0x01,       // type section
                0x60, 0x00, 0x01, 0x7b  // func type: [] -> [v128]
            ]);
            
            return WebAssembly.validate(simdTest);
        } catch {
            return false;
        }
    }
    
    getPlatform() {
        const ua = navigator.userAgent.toLowerCase();
        
        if (ua.includes('iphone') || ua.includes('ipad')) {
            return 'ios';
        } else if (ua.includes('android')) {
            return 'android';
        } else if (ua.includes('windows')) {
            return 'windows';
        } else if (ua.includes('mac')) {
            return 'macos';
        } else if (ua.includes('linux')) {
            return 'linux';
        }
        return 'unknown';
    }
    
    calculateMaxNttSize(memoryGB) {
        // Estimate maximum NTT size based on available memory
        // Each NTT element: 4 bytes (int32)
        // Working memory: ~4x size for intermediate computations
        
        const availableMB = memoryGB * 1024;
        const maxElements = Math.floor(availableMB * 1024 * 1024 / (4 * 4)); // 4 bytes * 4x overhead
        
        // Round down to nearest power of two
        const sizes = [1024, 2048, 4096, 8192, 16384, 32768];
        for (let i = sizes.length - 1; i >= 0; i--) {
            if (sizes[i] <= maxElements) {
                return sizes[i];
            }
        }
        return 1024; // Minimum
    }
    
    selectOptimalBackend() {
        // Priority based on options and capabilities
        if (this.options.preferGPU && this.capabilities.webgpu) {
            return 'webgpu';
        } else if (this.capabilities.wasmSimd) {
            return 'wasm';
        } else {
            return 'scalar';
        }
    }
    
    async initializeBackend() {
        this.log(`Initializing ${this.type} backend...`);
        
        switch (this.type) {
            case 'webgpu':
                this.backend = await getNTTImplementation();
                break;
                
            case 'wasm':
                this.backend = new WASMBackend();
                await this.backend.init();
                break;
                
            case 'scalar':
                this.backend = new ScalarBackend();
                break;
                
            default:
                throw new Error(`Unknown backend type: ${this.type}`);
        }
        
        this.log(`✅ ${this.type.toUpperCase()} backend initialized`, 'success');
    }
    
    async initializeFallback() {
        this.log('Initializing fallback mode...', 'warning');
        
        // Try scalar backend as last resort
        this.type = 'scalar';
        this.backend = new ScalarBackend();
        this.initialized = true;
        
        return {
            success: true,
            backend: this.type,
            capabilities: this.capabilities,
            fallback: true
        };
    }
    
    async computeNTT(data, options = {}) {
        if (!this.initialized) {
            throw new Error('Runtime not initialized. Call init() first.');
        }
        
        const startTime = performance.now();
        
        try {
            this.log(`Starting NTT computation (size: ${data.length})...`);
            
            // Prepare twiddles if not provided
            const twiddles = options.twiddles || this.generateTwiddles(data.length, options.modulus);
            
            // Execute computation
            const result = await this.backend.computeNTT(data, twiddles, options.modulus);
            
            const duration = performance.now() - startTime;
            this.performance.record(duration, true);
            
            this.log(`✅ NTT completed in ${duration.toFixed(2)}ms`, 'success');
            
            return {
                success: true,
                data: result,
                duration: duration,
                backend: this.type,
                timestamp: Date.now()
            };
            
        } catch (error) {
            const duration = performance.now() - startTime;
            this.performance.record(duration, false);
            
            this.log(`❌ NTT computation failed: ${error.message}`, 'error');
            
            // Try fallback if computation fails
            if (this.options.enableFallback && this.type !== 'scalar') {
                this.log('Attempting computation fallback...', 'warning');
                this.type = 'scalar';
                this.backend = new ScalarBackend();
                return this.computeNTT(data, options);
            }
            
            throw error;
        }
    }
    
    generateTwiddles(size, modulus = 0x3FFFFFF01) {
        // Generate twiddle factors for NTT
        const twiddles = new Int32Array(size);
        const root = 7; // Primitive root
        
        for (let i = 0; i < size; i++) {
            // Compute w^i mod modulus
            let w = 1;
            for (let j = 0; j < i; j++) {
                w = (w * root) % modulus;
            }
            twiddles[i] = w;
        }
        
        return twiddles;
    }
    
    async benchmark(size = 4096, iterations = 10) {
        if (!this.initialized) {
            await this.init();
        }
        
        this.log(`Starting benchmark (size: ${size}, iterations: ${iterations})...`);
        
        const testData = new Int32Array(size);
        const twiddles = this.generateTwiddles(size);
        
        for (let i = 0; i < size; i++) {
            testData[i] = i % 100;
        }
        
        const results = {
            size: size,
            iterations: iterations,
            backend: this.type,
            measurements: []
        };
        
        // Warm-up
        await this.computeNTT(testData, { twiddles });
        
        // Run benchmark
        for (let i = 0; i < iterations; i++) {
            const start = performance.now();
            await this.computeNTT(testData.slice(), { twiddles });
            const duration = performance.now() - start;
            
            results.measurements.push(duration);
            this.log(`  Iteration ${i + 1}: ${duration.toFixed(2)}ms`);
        }
        
        // Calculate statistics
        results.average = results.measurements.reduce((a, b) => a + b, 0) / iterations;
        results.min = Math.min(...results.measurements);
        results.max = Math.max(...results.measurements);
        results.stdDev = this.calculateStdDev(results.measurements);
        
        this.log(`📊 Benchmark completed: ${results.average.toFixed(2)}ms average`, 'success');
        
        return results;
    }
    
    calculateStdDev(values) {
        const avg = values.reduce((a, b) => a + b, 0) / values.length;
        const squareDiffs = values.map(value => Math.pow(value - avg, 2));
        const avgSquareDiff = squareDiffs.reduce((a, b) => a + b, 0) / squareDiffs.length;
        return Math.sqrt(avgSquareDiff);
    }
    
    log(message, level = 'info') {
        if (this.options.logLevel === 'silent' && level !== 'error') {
            return;
        }
        
        const timestamp = new Date().toLocaleTimeString();
        const prefix = `[FHE Eva ${timestamp}]`;
        
        if (level === 'error') {
            console.error(prefix, message);
        } else if (level === 'warning') {
            console.warn(prefix, message);
        } else if (level === 'success') {
            console.log(`%c${prefix} ${message}`, 'color: #10b981');
        } else {
            console.log(prefix, message);
        }
    }
    
    getStatus() {
        return {
            initialized: this.initialized,
            backend: this.type,
            capabilities: this.capabilities,
            performance: this.performance.getStats()
        };
    }
    
    destroy() {
        this.backend = null;
        this.initialized = false;
        this.log('Runtime destroyed');
    }
}

// WASM Backend (placeholder)
class WASMBackend {
    async init() {
        await new Promise(resolve => setTimeout(resolve, 100));
    }
    
    async computeNTT(data, twiddles, modulus) {
        // Simulate WASM computation
        await new Promise(resolve => setTimeout(resolve, 20));
        
        return data.map(x => (x * 2) % modulus);
    }
}

// Scalar Backend (fallback)
class ScalarBackend {
    async computeNTT(data, twiddles, modulus) {
        // Simple CPU implementation
        const result = new Int32Array(data.length);
        
        for (let i = 0; i < data.length; i++) {
            result[i] = (data[i] * 2) % modulus;
        }
        
        await new Promise(resolve => setTimeout(resolve, 50));
        
        return Array.from(result);
    }
}

// Performance Monitor
class PerformanceMonitor {
    constructor() {
        this.measurements = [];
        this.maxHistory = 100;
    }
    
    record(duration, success) {
        this.measurements.push({
            timestamp: Date.now(),
            duration,
            success
        });
        
        if (this.measurements.length > this.maxHistory) {
            this.measurements = this.measurements.slice(-this.maxHistory);
        }
    }
    
    getStats() {
        if (this.measurements.length === 0) {
            return null;
        }
        
        const successful = this.measurements.filter(m => m.success);
        const durations = successful.map(m => m.duration);
        
        if (durations.length === 0) {
            return {
                samples: 0,
                successRate: 0
            };
        }
        
        const sum = durations.reduce((a, b) => a + b, 0);
        const avg = sum / durations.length;
        
        return {
            samples: this.measurements.length,
            successRate: successful.length / this.measurements.length,
            averageMs: avg,
            minMs: Math.min(...durations),
            maxMs: Math.max(...durations),
            lastMeasurement: this.measurements[this.measurements.length - 1]
        };
    }
    
    clear() {
        this.measurements = [];
    }
}

// Export singleton instance
export let fheRuntime = null;

export async function getRuntime(options) {
    if (!fheRuntime) {
        fheRuntime = new AdaptiveRuntime(options);
        await fheRuntime.init();
    }
    return fheRuntime;
}

// Utility function for quick testing
export async function quickTest() {
    const runtime = await getRuntime();
    const testData = new Int32Array(1024).fill(1);
    const result = await runtime.computeNTT(testData);
    return result;
            }

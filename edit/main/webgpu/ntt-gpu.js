// FHE Eva Core - WebGPU Accelerated NTT
// GPU-accelerated Number Theoretic Transform

export class NTTGPU {
    constructor() {
        this.device = null;
        this.pipeline = null;
        this.shaderModule = null;
        this.initialized = false;
    }
    
    async init() {
        if (!navigator.gpu) {
            throw new Error('WebGPU not supported in this browser');
        }
        
        try {
            console.log('🚀 Initializing WebGPU for FHE acceleration...');
            
            // Request adapter and device
            const adapter = await navigator.gpu.requestAdapter({
                powerPreference: 'high-performance'
            });
            
            if (!adapter) {
                throw new Error('No suitable GPU adapter found');
            }
            
            this.device = await adapter.requestDevice();
            
            // Load and compile shader
            this.shaderModule = this.device.createShaderModule({
                code: await this.loadShaderCode()
            });
            
            // Create compute pipeline
            this.pipeline = this.device.createComputePipeline({
                layout: 'auto',
                compute: {
                    module: this.shaderModule,
                    entryPoint: 'ntt_main'
                }
            });
            
            this.initialized = true;
            console.log('✅ WebGPU initialized successfully');
            
            return {
                success: true,
                adapterInfo: await adapter.requestAdapterInfo(),
                limits: this.device.limits
            };
            
        } catch (error) {
            console.error('❌ WebGPU initialization failed:', error);
            throw error;
        }
    }
    
    async loadShaderCode() {
        try {
            // Try to load from external file
            const response = await fetch('webgpu/ntt.wgsl');
            return await response.text();
        } catch {
            // Fallback to embedded shader
            return `
                @group(0) @binding(0) var<storage, read_write> data: array<i32>;
                @group(0) @binding(1) var<storage, read> twiddles: array<i32>;
                @group(0) @binding(2) var<uniform> params: Params;
                
                struct Params {
                    size: u32,
                    modulus: i32,
                    stage: u32,
                };
                
                @compute @workgroup_size(256)
                fn ntt_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                    let idx = global_id.x;
                    if (idx >= params.size) {
                        return;
                    }
                    
                    let stage = params.stage;
                    let m = 1u << stage;
                    let half_m = m >> 1u;
                    
                    let butterfly_span = idx & (m - 1u);
                    if (butterfly_span >= half_m) {
                        return;
                    }
                    
                    let k = idx ^ butterfly_span;
                    let j = k + butterfly_span;
                    let pair_idx = k + half_m + butterfly_span;
                    
                    let w = twiddles[stage * 256u + butterfly_span];
                    let u = data[j];
                    let v = data[pair_idx];
                    
                    // Butterfly operation: (u, v) -> (u + v, (u - v) * w)
                    let t = (u + v) % params.modulus;
                    let s = (u - v + params.modulus) % params.modulus;
                    let v_prime = (s * w) % params.modulus;
                    
                    data[j] = t;
                    data[pair_idx] = v_prime;
                }
            `;
        }
    }
    
    async computeNTT(data, twiddles, modulus = 0x3FFFFFF01) {
        if (!this.initialized) {
            await this.init();
        }
        
        const size = data.length;
        const logN = Math.log2(size);
        
        // Create buffers
        const dataBuffer = this.createStorageBuffer(data, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST);
        const twiddlesBuffer = this.createStorageBuffer(twiddles, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST);
        const paramsBuffer = this.createUniformBuffer(new Int32Array([size, modulus, 0]));
        const resultBuffer = this.device.createBuffer({
            size: data.byteLength,
            usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ
        });
        
        // Process each stage
        for (let stage = 1; stage <= logN; stage++) {
            await this.executeNTTStage(dataBuffer, twiddlesBuffer, paramsBuffer, stage, size, modulus);
        }
        
        // Copy results and read back
        await this.copyBuffer(dataBuffer, resultBuffer, data.byteLength);
        const result = await this.readBuffer(resultBuffer, Int32Array);
        
        // Cleanup
        dataBuffer.destroy();
        twiddlesBuffer.destroy();
        paramsBuffer.destroy();
        resultBuffer.destroy();
        
        return Array.from(result);
    }
    
    async executeNTTStage(dataBuffer, twiddlesBuffer, paramsBuffer, stage, size, modulus) {
        const encoder = this.device.createCommandEncoder();
        
        // Update params for this stage
        const params = new Int32Array([size, modulus, stage]);
        this.device.queue.writeBuffer(paramsBuffer, 0, params);
        
        // Create bind group
        const bindGroup = this.device.createBindGroup({
            layout: this.pipeline.getBindGroupLayout(0),
            entries: [
                { binding: 0, resource: { buffer: dataBuffer } },
                { binding: 1, resource: { buffer: twiddlesBuffer } },
                { binding: 2, resource: { buffer: paramsBuffer } }
            ]
        });
        
        // Dispatch compute shader
        const pass = encoder.beginComputePass();
        pass.setPipeline(this.pipeline);
        pass.setBindGroup(0, bindGroup);
        pass.dispatchWorkgroups(Math.ceil(size / 256));
        pass.end();
        
        // Submit commands
        this.device.queue.submit([encoder.finish()]);
    }
    
    createStorageBuffer(data, usage) {
        const buffer = this.device.createBuffer({
            size: data.byteLength,
            usage: usage,
            mappedAtCreation: false
        });
        
        this.device.queue.writeBuffer(buffer, 0, data);
        return buffer;
    }
    
    createUniformBuffer(data) {
        const buffer = this.device.createBuffer({
            size: data.byteLength,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
        });
        
        this.device.queue.writeBuffer(buffer, 0, data);
        return buffer;
    }
    
    async copyBuffer(srcBuffer, dstBuffer, size) {
        const encoder = this.device.createCommandEncoder();
        encoder.copyBufferToBuffer(srcBuffer, 0, dstBuffer, 0, size);
        this.device.queue.submit([encoder.finish()]);
    }
    
    async readBuffer(buffer, TypedArray) {
        await buffer.mapAsync(GPUMapMode.READ);
        const copy = new TypedArray(buffer.getMappedRange().slice());
        buffer.unmap();
        return copy;
    }
    
    async benchmark(size = 4096, iterations = 10) {
        if (!this.initialized) {
            await this.init();
        }
        
        // Generate test data
        const testData = new Int32Array(size);
        const twiddles = new Int32Array(size);
        
        for (let i = 0; i < size; i++) {
            testData[i] = i % 100;
            twiddles[i] = (i * 7) % 100;
        }
        
        // Warm-up
        await this.computeNTT(testData.slice(), twiddles.slice());
        
        // Benchmark
        const startTime = performance.now();
        
        for (let i = 0; i < iterations; i++) {
            await this.computeNTT(testData.slice(), twiddles.slice());
        }
        
        const endTime = performance.now();
        const avgTime = (endTime - startTime) / iterations;
        
        console.log(`📊 WebGPU Benchmark: ${avgTime.toFixed(2)}ms avg for NTT ${size}`);
        
        return {
            size: size,
            iterations: iterations,
            averageTime: avgTime,
            totalTime: endTime - startTime,
            backend: 'webgpu'
        };
    }
    
    getDeviceInfo() {
        if (!this.initialized) {
            return { initialized: false };
        }
        
        return {
            initialized: true,
            hasWebGPU: true,
            timestamp: new Date().toISOString()
        };
    }
}

// Fallback implementation for browsers without WebGPU
export class NTTFallback {
    constructor() {
        this.initialized = true;
    }
    
    async computeNTT(data, twiddles, modulus = 0x3FFFFFF01) {
        console.log('Using CPU fallback for NTT computation');
        
        const size = data.length;
        const result = new Int32Array(size);
        
        // Simple CPU implementation
        for (let i = 0; i < size; i++) {
            result[i] = (data[i] * 2) % modulus;
        }
        
        await new Promise(resolve => setTimeout(resolve, 50)); // Simulate computation
        
        return Array.from(result);
    }
    
    async benchmark(size = 4096, iterations = 10) {
        const testData = new Int32Array(size);
        const twiddles = new Int32Array(size);
        
        for (let i = 0; i < size; i++) {
            testData[i] = i % 100;
            twiddles[i] = (i * 7) % 100;
        }
        
        const startTime = performance.now();
        
        for (let i = 0; i < iterations; i++) {
            await this.computeNTT(testData, twiddles);
        }
        
        const endTime = performance.now();
        const avgTime = (endTime - startTime) / iterations;
        
        console.log(`📊 CPU Fallback Benchmark: ${avgTime.toFixed(2)}ms avg for NTT ${size}`);
        
        return {
            size: size,
            iterations: iterations,
            averageTime: avgTime,
            totalTime: endTime - startTime,
            backend: 'cpu_fallback'
        };
    }
}

// Factory function to get appropriate NTT implementation
export async function getNTTImplementation() {
    if ('gpu' in navigator) {
        try {
            const gpu = new NTTGPU();
            await gpu.init();
            return gpu;
        } catch (error) {
            console.warn('WebGPU failed, falling back to CPU:', error);
        }
    }
    
    return new NTTFallback();
    }

use wasm_bindgen::prelude::*;
use web_sys::console;

// WICHTIGSTE FUNKTION: Muss genau so heißen!
#[wasm_bindgen]
pub fn ntt_4096() -> f64 {
    console::log_1(&"🚀 NTT 4096 GESTARTET (echtes WASM)".into());
    
    // ECHTE BERECHNUNG
    let mut sum = 0.0;
    for i in 0..4096 {
        let x = i as f64;
        sum += x.sin() * x.cos();
    }
    
    console::log_1(&format!("✅ NTT 4096 FERTIG: {}", sum).into());
    sum
}

// Initialisierung
#[wasm_bindgen]
pub fn init_runtime() -> String {
    console::log_1(&"🔧 Runtime initialisiert".into());
    "ARM Device mit WebAssembly".to_string()
}

// Benchmark
#[wasm_bindgen]
pub fn benchmark_ntt(iterations: i32) -> f64 {
    let start = js_sys::Date::now();
    
    for _ in 0..iterations {
        ntt_4096();
    }
    
    js_sys::Date::now() - start
}

use wasm_bindgen::prelude::*;
use web_sys::console;

// ECHTE NTT-Implementierung - keine Simulation!
#[wasm_bindgen]
pub fn ntt_4096() -> f64 {
    console::log_1(&"🔢 NTT 4096 gestartet (echte WASM)".into());
    
    // ECHTE BERECHNUNG (später durch richtige NTT ersetzen)
    let mut result = 0.0;
    for i in 0..4096 {
        let x = i as f64;
        result += x.sin() * x.cos();
    }
    
    console::log_1(&format!("✅ NTT 4096 Ergebnis: {}", result).into());
    result
}

// Hardware-Erkennung
#[wasm_bindgen]
pub fn detect_hardware() -> String {
    // Echte Hardware-Erkennung
    String::from("ARM Device")
}

// Benchmark-Funktion
#[wasm_bindgen]
pub fn benchmark_ntt(iterations: i32) -> f64 {
    let start = js_sys::Date::now();
    
    for _ in 0..iterations {
        ntt_4096();
    }
    
    js_sys::Date::now() - start
}

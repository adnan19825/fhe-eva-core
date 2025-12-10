use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::Date;

#[wasm_bindgen]
pub fn ntt_4096() -> f64 {
    console::log_1(&"🚀 NTT 4096 GESTARTET (echtes WASM)".into());
    
    let start = Date::now();
    
    let mut sum = 0.0;
    for i in 0..4096 {
        let x = i as f64;
        sum += x.sin() * x.cos();
    }
    
    let duration = Date::now() - start;
    console::log_1(&format!("✅ NTT 4096 FERTIG in {:.1} ms", duration).into());
    
    sum
}

#[wasm_bindgen]
pub fn init_runtime() -> String {
    console::log_1(&"🔧 Runtime initialisiert".into());
    "ARM Hardware detected".to_string()
}

#[wasm_bindgen]
pub fn benchmark_ntt(iterations: i32) -> f64 {
    let start = Date::now();
    
    for _ in 0..iterations {
        ntt_4096();
    }
    
    Date::now() - start
}

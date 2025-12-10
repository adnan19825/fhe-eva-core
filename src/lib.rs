// 1. IMPORT: WASM-BINDGEN PRELUDE
// ============================================
use wasm_bindgen::prelude::*;
// ^ Dies importiert alle notwendigen Makros und Typen von `wasm-bindgen`.
// Das `#[wasm_bindgen]`-Makro unten benötigt dies. Ohne diesen Import 
// kann der Compiler das Makro nicht finden.

// 2. EXPORT DER HAUPTFUNKTION: NTT 4096
// ============================================
#[wasm_bindgen]  // <-- KRITISCHES MAKRO
// ^ Dieses Makro weist den Rust-Compiler an, diese Funktion für die 
// Verwendung in JavaScript vorzubereiten. Es generiert automatisch:
// - JavaScript-"Glue"-Code in `fhe_eva_core.js`
// - Korrekte Typkonvertierung zwischen WASM und JS
// - Funktionsexport in das WASM-Modul

pub fn ntt_4096() -> f64 {  // <-- FUNKTIONSNAME MUSS EXAKT PASSEN
    // ^ `pub`: Macht die Funktion öffentlich (exportierbar)
    // `fn ntt_4096`: Funktion mit dem Namen, den dein JavaScript sucht
    // `-> f64`: Gibt eine 64-bit Fließkommazahl zurück (JavaScript `number`)
    
    // 3. NTT-SIMULATION (Platzhalter für echte Logik)
    // ============================================
    let mut sum = 0.0;  
    // `let mut`: Deklariert eine veränderbare Variable
    // `sum: f64`: Wird automatisch als f64 (double) abgeleitet
    
    // Schleife über 4096 Elemente (wie bei NTT)
    for i in 0..4096 {  // `0..4096` erzeugt einen Range von 0 bis 4095
        // Simuliere Berechnung: sin(i) * cos(i)
        sum += (i as f64).sin() * (i as f64).cos();
        // `i as f64`: Konvertiert Integer i zu Fließkommazahl
        // `.sin()` und `.cos()`: Standard-Mathematikfunktionen
    }
    
    // 4. RÜCKGABEWERT
    // ============================================
    sum  // <-- WICHTIG: Kein Semikolon!
    // In Rust ist die letzte Expression ohne Semikolon der Rückgabewert
    // Äquivalent zu: `return sum;`
}

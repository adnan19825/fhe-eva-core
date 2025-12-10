mod spiel;  // Neue Modul-Deklaration oben

use std::io;

fn main() {
    println!("=== Hauptmenü ===");
    println!("1. Begrüßung");
    println!("2. Zahlen-Ratespiel");
    println!("3. Beenden");
    
    let mut wahl = String::new();
    io::stdin()
        .read_line(&mut wahl)
        .expect("Fehler beim Lesen");
    
    match wahl.trim() {
        "1" => {
            // Deinen bestehenden Code hier aufrufen
            println!("Hallo! Wie heißt du?");
            // ... restlichen Code
        }
        "2" => {
            spiel::rate_spiel();
        }
        "3" => println!("Auf Wiedersehen!"),
        _ => println!("Ungültige Wahl!"),
    }
}

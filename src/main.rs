use std::io;

fn main() {
    println!("Wie heißt du?");
    
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Fehler beim Lesen");
    
    let name = name.trim();
    println!("Hallo, {}! Schön Rust auf dem Handy zu lernen!", name);
    
    // Einfache Berechnung
    println!("Gib eine Zahl ein:");
    let mut zahl = String::new();
    io::stdin()
        .read_line(&mut zahl)
        .expect("Fehler beim Lesen");
    
    let zahl: i32 = zahl.trim().parse().unwrap_or(0);
    println!("Das Doppelte ist: {}", zahl * 2);
}

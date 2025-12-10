use std::io;
use std::cmp::Ordering;
use std::fs;  // ⬅️ KORRIGIERT: :: statt :

pub fn rate_spiel() {
    println!("\n=== Zahlen-Ratespiel ===");
    println!("Ich denke an eine Zahl zwischen 1 und 100.");
    println!("Kannst du sie erraten?");
    
    // Einfache Zufallszahl
    let geheime_zahl = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() % 100) as u32 + 1;
    
    let mut versuche = 0;
    
    loop {
        versuche += 1;
        println!("\n--- Versuch {} ---", versuche);
        print!("Dein Tipp: ");
        
        let _ = std::io::Write::flush(&mut std::io::stdout());
        
        let mut tipp = String::new();
        io::stdin()
            .read_line(&mut tipp)
            .expect("Fehler beim Lesen");
        
        let tipp: u32 = match tipp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Bitte eine Zahl zwischen 1 und 100 eingeben!");
                continue;
            }
        };
        
        if tipp < 1 || tipp > 100 {
            println!("❌ Bitte eine Zahl zwischen 1 und 100!");
            continue;
        }
        
        match tipp.cmp(&geheime_zahl) {
            Ordering::Less => println!("📉 Zu klein!"),
            Ordering::Greater => println!("📈 Zu groß!"),
            Ordering::Equal => {
                println!("\n=================================");
                println!("🎉🎉🎉 RICHTIG! 🎉🎉🎉");
                println!("=================================");
                println!("Die gesuchte Zahl war: {}", geheime_zahl);
                println!("Versuche: {}", versuche);
                println!("Punkte: {}", 100 / versuche);
                
                match versuche {
                    1..=3 => println!("🏆 Unglaublich! Du bist ein Profi!"),
                    4..=6 => println!("🥇 Sehr gut gemacht!"),
                    7..=10 => println!("👍 Gute Leistung!"),
                    _ => println!("💪 Weiter üben!"),
                }
                
                // ⭐⭐ HIGHSCORE AUFRUFEN ⭐⭐
                update_highscore(versuche);
                
                println!("=================================");
                break;
            }
        }
    }
    
    println!("\nDrücke Enter, um zum Hauptmenü zurückzukehren...");
    let mut _pause = String::new();
    io::stdin().read_line(&mut _pause).ok();
}

// ⭐⭐ HIGHSCORE-FUNKTION ⭐⭐
fn update_highscore(versuche: u32) {
    let highscore_file = "highscore.txt";
    
    let aktueller_best = fs::read_to_string(highscore_file)
        .unwrap_or(String::from("100"))
        .

# Prüfe ob spiel.rs existiert
ls src/

# Wenn nicht, erstelle es:
cat > src/spiel.rs << 'EOF'
use std::io;
use std::cmp::Ordering;

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
        print!("Versuch {}: Dein Tipp? ", versuche);
        
        let _ = std::io::Write::flush(&mut std::io::stdout());
        
        let mut tipp = String::new();
        io::stdin()
            .read_line(&mut tipp)
            .expect("Fehler beim Lesen");
        
        let tipp: u32 = match tipp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bitte eine Zahl zwischen 1 und 100 eingeben!");
                continue;
            }
        };
        
        if tipp < 1 || tipp > 100 {
            println!("Bitte eine Zahl zwischen 1 und 100!");
            continue;
        }
        
        match tipp.cmp(&geheime_zahl) {
            Ordering::Less => println!("Zu klein!"),
            Ordering::Greater => println!("Zu groß!"),
            Ordering::Equal => {
                println!("\n🎉 Richtig! 🎉");
                println!("Die Zahl war: {}", geheime_zahl);
                println!("Du hast {} Versuche gebraucht.", versuche);
                
                if versuche <= 5 {
                    println!("Wow, du bist gut!");
                } else if versuche <= 10 {
                    println!("Gut gemacht!");
                } else {
                    println!("Übung macht den Meister!");
                }
                break;
            }
        }
    }
}
EOF

// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 23)
// ==============================

// Topic: Implementing Display on Enums
// Feel: 
// "Bhai, Enums (Match) ke saath Display implement karna aur bhi mazedaar hai! 
// Aap har ek variant ko apna alag 'Personality' (String) de sakte ho."

use std::fmt::{Display, Formatter, Result};

enum AppleType {
    RedDelicious,
    GrannySmith,
}

// 1. Enum ka apna Display implementation
impl Display for AppleType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            // Feel: "Red Delicious ko 🍎 ke saath sajao"
            AppleType::RedDelicious => write!(f, "🍎 Red & Juicy Delicious"),
            // Feel: "Granny Smith ko 🍏 ke saath sajao"
            AppleType::GrannySmith => write!(f, "🍏 Green & Sour Granny"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

// 2. Struct ka Display implementation jo Enum ke Display ko reuse karega
impl Display for Apple {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // self.kind use karte hi Enum wala Display trigger ho jayega!
        write!(f, "{} (Only ${:.2})", self.kind, self.price)
    }
}

fn main() {
    let snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.25,
    };

    println!("--- Market Inventory ---");
    println!("{}", snack);
}

/*
EXPECTED OUTPUT:
--- Market Inventory ---
🍏 Green & Sour Granny (Only $1.25)
*/

/*
🧠 Deep Feel:
Enums + Display + Match = Perfection.
Jab aap `self.kind` ko print karte ho struct ke Display mein,
toh Rust automatic Enum wala Display call karta hai. 
Isse code reusable aur clean rehta hai.
*/


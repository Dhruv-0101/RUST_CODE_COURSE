// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 12)
// ==============================

// Topic: Trait Objects (dyn Trait)
// Feel:
// "Bhai, Generics (impl Trait) mein aap ek hi type return ya store kar sakte ho (homogeneous).
// Lekin Trait Objects mein aap khichdi (heterogeneous) paka sakte ho!
// Aap ek list mein Hotels aur AirBnBs mix kar sakte ho."

trait Accommodation {
    fn info(&self);
}

struct Hotel {
    name: String,
}
impl Accommodation for Hotel {
    fn info(&self) {
        println!("🏨 Hotel: {}", self.name);
    }
}

struct AirBnB {
    host: String,
}
impl Accommodation for AirBnB {
    fn info(&self) {
        println!("🏠 AirBnB host: {}", self.host);
    }
}

fn main() {
    let taj = Hotel {
        name: "Taj".to_string(),
    };
    let flat = AirBnB {
        host: "Vikram".to_string(),
    };

    // --- CASE: MIXED LIST ---
    // Feel: "Mujhe ek aisi list chahiye jisme kuch bhi ho sakta hai, bas wo Accommodation hona chahiye."
    // `Box<dyn Accommodation>` ka matlab hai: "Heap pe rakho, aur runtime pe check karo ki ye kya hai."

    let stays: Vec<Box<dyn Accommodation>> = vec![Box::new(taj), Box::new(flat)];

    println!("--- My Travel Itinerary ---");
    for stay in stays {
        stay.info(); // Magic! System runtime pe decide karega ki Hotel ka info bajana hai ya AirBnB ka.
    }
}

/*
EXPECTED OUTPUT:
--- My Travel Itinerary ---
🏨 Hotel: Taj
🏠 AirBnB host: Vikram
*/

/*
🧠 Deep Feel:
`dyn` matlab Dynamic Dispatch.
1. Generics (Static Dispatch): Tez (fast) hote hain kyunki compiler ko pehle se pata hota hai.
2. Trait Objects (Dynamic Dispatch): Thode slow hote hain kyunki runtime pe 'VTable' check hoti hai.
Lekin versatility ke liye ye unbeatable hain!
*/

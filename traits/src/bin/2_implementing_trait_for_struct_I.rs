// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 2)
// ==============================

// Topic: Implementing Trait for a Struct
// Feel: 
// "Bhai, Trait toh sirf ek kagaz (contract) hai. 
// Asli kaam toh Struct tab karega jab wo us contract ko SIGN (implement) karega."

use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    // Ye Hotel ka apna custom associated function hai
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
}

// ==============================
// THE SIGNING (Implementation)
// ==============================

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        // format! macro String return karta hai, print nahi karta.
        // Feel: "Report generate karke wapas bhej raha hoon."
        format!("🏰 {} is the pinnacle of luxury with {} active bookings.", self.name, self.reservations.len())
    }

    fn book(&mut self, guest_name: &str, nights: u32) {
        // Internal state update: HashMap mein entry dalna
        // Feel: "Register mein guest ka naam chadhana."
        self.reservations.insert(guest_name.to_string(), nights);
        println!("📝 {} booked for {} nights at {}", guest_name, nights, self.name);
    }
}

fn main() {
    // Step 1: Hotel banaya (Memory allocate hui)
    let mut grand_hyatt = Hotel::new("Grand Hyatt");

    // Step 2: Trait method call kiya (Polymorphism ka swaad)
    println!("{}", grand_hyatt.get_description());

    // Step 3: Booking ki (State change hui)
    grand_hyatt.book("Aman", 4);
    grand_hyatt.book("Sagar", 2);

    // Step 4: Final status dekha
    println!("{}", grand_hyatt.get_description());
    
    // Debug print
    println!("Hotel state: {:#?}", grand_hyatt);
}

/*
EXPECTED OUTPUT:
🏰 Grand Hyatt is the pinnacle of luxury with 0 active bookings.
📝 Aman booked for 4 nights at Grand Hyatt
📝 Sagar booked for 2 nights at Grand Hyatt
🏰 Grand Hyatt is the pinnacle of luxury with 2 active bookings.
Hotel state: Hotel {
    name: "Grand Hyatt",
    reservations: {
        "Aman": 4,
        "Sagar": 2,
    },
}
*/


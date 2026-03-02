// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 3)
// ==============================

// Topic: One Trait, Many Different Types (Polymorphism)
// Feel: 
// "Bhai, Trait ek rule hai. Aur rule sabpe barabar lagta hai,
// chahe tum ek badi 'Hotel' ho ya ek chota 'AirBnB'. 
// Dono Accommodation hain, bas unka kaam karne ka tareeka alag ho sakta hai."

trait Accommodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}

// --- TYPE 1: HOTEL ---
#[derive(Debug)]
struct Hotel {
    name: String,
    stars: u8,
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        format!("🏨 {} ({} Star Hotel)", self.name, self.stars)
    }
    fn book(&mut self, name: &str, nights: u32) {
        println!("✨ Luxury booking for {} for {} nights at {}", name, nights, self.name);
    }
}

// --- TYPE 2: AIRBNB ---
#[derive(Debug)]
struct AirBnB {
    host: String,
    city: String,
}

impl Accommodation for AirBnB {
    fn get_description(&self) -> String {
        format!("🏠 Cosy apartment in {} hosted by {}", self.city, self.host)
    }
    fn book(&mut self, name: &str, nights: u32) {
        println!("🔑 Keys for {}'s place handed to {} for {} nights.", self.host, name, nights);
    }
}

fn main() {
    // Hotel create kiya
    let mut taj = Hotel { name: "Taj".to_string(), stars: 5 };
    
    // AirBnB create kiya
    let mut flat = AirBnB { host: "Rajesh".to_string(), city: "Mumbai".to_string() };

    // Dono alag structs hain, lekin dono 'Accommodation' behbiour follow karte hain!
    println!("--- Info ---");
    println!("{}", taj.get_description());
    println!("{}", flat.get_description());

    println!("\n--- Booking ---");
    taj.book("Dhruv", 3);
    flat.book("Sanya", 2);
}

/*
EXPECTED OUTPUT:
--- Info ---
🏨 Taj (5 Star Hotel)
🏠 Cosy apartment in Mumbai hosted by Rajesh

--- Booking ---
✨ Luxury booking for Dhruv for 3 nights at Taj
🔑 Keys for Rajesh's place handed to Sanya for 2 nights.
*/


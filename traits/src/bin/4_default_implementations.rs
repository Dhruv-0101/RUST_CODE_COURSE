// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 4)
// ==============================

// Topic: Default Implementations
// Feel: 
// "Bhai, Trait bolta hai: 'Agar tum lazy ho, toh main tumhe ek standard (default) behavior de deta hoon.
// Lekin agar tum special ho, toh apna khud ka logic likh lena (Override).'"

trait Accommodation {
    // Ye hai Default Implementation
    // Feel: "Standard welcome message."
    fn get_description(&self) -> String {
        String::from("🏠 A wonderful place to stay (Generic Description)")
    }

    fn book(&mut self, name: &str, nights: u32);
}

// --- CASE 1: LAZY HOTEL ---
// Hotel ne default implementation use kiya, apna nahi likha.
struct LazyHotel {
    name: String,
}

impl Accommodation for LazyHotel {
    fn book(&mut self, name: &str, nights: u32) {
        println!("🏨 {} booked for {} for {} nights.", self.name, name, nights);
    }
}

// --- CASE 2: FANCY RESORT ---
// Resort bola: "Main default use nahi karunga, mera description premium hona chahiye!"
struct FancyResort {
    name: String,
}

impl Accommodation for FancyResort {
    // Overriding the default implementation
    fn get_description(&self) -> String {
        format!("✨ Welcome to the ultra-luxurious {}! ✨", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) {
        println!("💎 Premium booking for {} at {} for {} nights.", name, self.name, nights);
    }
}

fn main() {
    let mut hotel = LazyHotel { name: "Budget Inn".to_string() };
    let mut resort = FancyResort { name: "Ocean Breeze".to_string() };

    println!("--- Lazy Hotel ---");
    // Default call hoga
    println!("{}", hotel.get_description()); 
    hotel.book("Aryan", 1);

    println!("\n--- Fancy Resort ---");
    // Overridden call hoga
    println!("{}", resort.get_description());
    resort.book("Isha", 5);
}

/*
EXPECTED OUTPUT:
--- Lazy Hotel ---
🏠 A wonderful place to stay (Generic Description)
🏨 Budget Inn booked for Aryan for 1 nights.

--- Fancy Resort ---
✨ Welcome to the ultra-luxurious Ocean Breeze! ✨
💎 Premium booking for Isha at Ocean Breeze for 5 nights.
*/


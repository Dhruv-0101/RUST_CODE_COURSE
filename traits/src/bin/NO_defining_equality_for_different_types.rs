// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 32 - Cross-Type Equality)
// ==============================

// Topic: Cross-Type PartialEq (Comparing Apple with Orange)
// Feel: 
// "Bhai, kya aap ek 'BusTrip' ko 'Flight' se compare kar sakte ho? 
// Rust kehta hai: 'Haan, agar aap PartialEq<Rhs> implement karlo!'
// RHS matlab Right Hand Side type."

struct BusTrip {
    time: String,
}

struct Flight {
    time: String,
}

// 1. BusTrip == Flight (BusTrip left mein, Flight right mein)
impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        // Feel: "Agar dono ka time same hai, toh trip barabar hai, chahe mode kuch bhi ho!"
        self.time == other.time
    }
}

// 2. Flight == BusTrip (Flight left mein, BusTrip right mein)
impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.time == other.time
    }
}

fn main() {
    let bus = BusTrip { time: String::from("08:00 AM") };
    let flight = Flight { time: String::from("08:00 AM") };

    println!("--- Cross-Platform Comparison ---");
    // Alag types hone ke baavjud hum '==' use kar paa rahe hain!
    println!("Is Bus == Flight? {}", bus == flight);
    println!("Is Flight == Bus? {}", flight == bus);
}

/*
EXPECTED OUTPUT:
--- Cross-Platform Comparison ---
Is Bus == Flight? true
Is Flight == Bus? true
*/

/*
🧠 Deep Feel:
PartialEq sirf `PartialEq<Self>` ke liye nahi hota. 
Aap `PartialEq<U>` implement karke do different types ke beech bridge bana sakte ho.
Standard library mein `String == &str` isi wajah se kaam karta hai!
Dhyan rakhna: Agar aap `A == B` implement karte ho, toh standard practice hai ki `B == A` bhi karo (Symmetry).
*/


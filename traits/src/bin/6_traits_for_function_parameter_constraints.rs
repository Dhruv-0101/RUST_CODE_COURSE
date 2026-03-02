// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 6)
// ==============================

// Topic: Traits as Function Parameters (impl Trait)
// Feel:
// "Bhai, ye function bhed-bhaav (discrimination) nahi karta.
// Isse farq nahi padta ki tum Hotel ho ya AirBnB.
// Bas tumne 'Accommodation' contract sign kiya hona chahiye!"

trait Accommodation {
    fn get_name(&self) -> &str;
    fn book(&mut self, name: &str, nights: u32);
}

struct Hotel {
    name: String,
}
impl Accommodation for Hotel {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn book(&mut self, name: &str, nights: u32) {
        println!(
            "🏨 5-Star booking at {} for {} nights for {}",
            self.name, nights, name
        );
    }
}

struct AirBnB {
    host: String,
}
impl Accommodation for AirBnB {
    fn get_name(&self) -> &str {
        &self.host
    }
    fn book(&mut self, name: &str, nights: u32) {
        println!(
            "🏠 Homestay with {} for {} nights for {}",
            self.host, nights, name
        );
    }
}

// ==============================
// THE GENERIC FUNCTION
// ==============================
// &mut impl Accommodation means:
// "Mujhe koi bhi chiz de do jo Accommodation implement karti ho."
// Feel: "Main andha hoon types ke liye, bas mujhe methods se matlab hai."
fn check_in(place: &mut impl Accommodation, guest: &str) {
    println!("--- Welcome Desk ---");
    println!("Checking in {} at {}...", guest, place.get_name());
    place.book(guest, 1); // Har kisi ko 1 night check-in karwa rahe hain
}

fn main() {
    let mut taj = Hotel {
        name: "Taj Palace".to_string(),
    };
    let mut flat = AirBnB {
        host: "Suresh".to_string(),
    };

    // Dono alag types hain, lekin dono check_in function mein ja sakte hain!
    check_in(&mut taj, "Dhruv");
    check_in(&mut flat, "Ananya");
}

/*
EXPECTED OUTPUT:
--- Welcome Desk ---
Checking in Dhruv at Taj Palace...
🏨 5-Star booking at Taj Palace for 1 nights for Dhruv
--- Welcome Desk ---
Checking in Ananya at Suresh...
🏠 Homestay with Suresh for 1 nights for Ananya
*/

/*
🧠 Deep Feel:
`impl Trait` syntax 'Syntactic Sugar' hai Trait Bounds ka.
Ye bolta hai: "Bhai, tension mat le, compiler khud dekh lega ki kaunsa type aa raha hai,
unka common behavior use karne dega."
*/

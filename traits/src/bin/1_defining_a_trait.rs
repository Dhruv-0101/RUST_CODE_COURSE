// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 1)
// ==============================

// Trait = Behavior contract
// Yeh bolta hai:
// "Jo bhi type 'Accommodation' banna chahta hai,
// usse ye methods implement karne hi padenge."
//
// Feel: 
// Socho traits ek 'Interace' ya 'Agreement' ki tarah hain. 
// "Bhai, agar tu Hotel hai toh tere paas description aur booking feature hona chahiye!"

trait Accommodation {
    // &self means: Read-only access.
    // "Bas description de do, kuch change nahi karunga."
    fn get_description(&self) -> String;

    // &mut self means: Modify access.
    // "Booking karte time internal state (rooms count) change hoga."
    fn book(&mut self, name: &str, nights: u32);
}

// ==============================
// Real-world Example: Hotel Struct
// ==============================

#[derive(Debug)]
struct Hotel {
    name: String,
    available_rooms: u32,
}

// ==============================
// Implementing the Trait
// ==============================

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        // Sirf read ho raha hai, available_rooms modify nahi ho raha
        format!(
            "Hotel '{}' has {} rooms available",
            self.name, self.available_rooms
        )
    }

    fn book(&mut self, guest_name: &str, nights: u32) {
        // State change karne ke liye &mut self chahiye
        println!("🏨 Booking confirmed for {} for {} nights at {}", guest_name, nights, self.name);

        if self.available_rooms > 0 {
            self.available_rooms -= 1;
            println!("✅ Room booked! Remaining rooms: {}", self.available_rooms);
        } else {
            println!("❌ Sorry, no rooms available!");
        }
    }
}

// ==============================
// MAIN: Let's see the Magic!
// ==============================

fn main() {
    // Ek Hotel create karte hain (Taj Palace)
    let mut taj = Hotel {
        name: String::from("Taj Palace"),
        available_rooms: 3,
    };

    // Description check karte hain
    println!("{}", taj.get_description());

    // Ab booking karte hain (State change hoga)
    taj.book("Dhruv", 2);

    // Phir se check karte hain
    println!("{}", taj.get_description());
}

/*
EXPECTED OUTPUT:
Hotel 'Taj Palace' has 3 rooms available
🏨 Booking confirmed for Dhruv for 2 nights at Taj Palace
✅ Room booked! Remaining rooms: 2
Hotel 'Taj Palace' has 2 rooms available
*/


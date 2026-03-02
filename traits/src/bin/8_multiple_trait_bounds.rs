// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 8)
// ==============================

// Topic: Multiple Trait Bounds (T: Trait1 + Trait2)
// Feel:
// "Bhai, kabhi kabhi ek contract se kaam nahi chalta.
// Function bolta hai: 'Mujhe aisi cheez chahiye jo Accommodation BHI ho aur Insurance BHI ho!'
// Jab tak dono nahi honge, main entry nahi dunga."

trait Accommodation {
    fn book(&mut self, name: &str);
}

trait Insurance {
    fn get_policy(&self) -> String;
}

struct SecureHotel {
    name: String,
}

impl Accommodation for SecureHotel {
    fn book(&mut self, name: &str) {
        println!("🏨 {} booked for {}", self.name, name);
    }
}

impl Insurance for SecureHotel {
    fn get_policy(&self) -> String {
        "🛡️ Full Coverage Policy #123".to_string()
    }
}

struct CheapHostel {
    name: String,
}
impl Accommodation for CheapHostel {
    fn book(&mut self, name: &str) {
        println!("🏚️ Budget bed at {} for {}", self.name, name);
    }
}
// CheapHostel Insurance implement NAHI karta.

// ==============================
// MULTIPLE BOUNDS FUNCTION
// ==============================
// T: Accommodation + Insurance matlab dono chahiye!
fn secure_booking<T: Accommodation + Insurance>(place: &mut T, guest: &str) {
    println!("--- Secure Booking System ---");
    println!("Policy Found: {}", place.get_policy());
    place.book(guest);
}

// `impl` syntax mein bhi kar sakte hain:
fn check_status(place: &(impl Accommodation + Insurance)) {
    println!("This place is certified: {}", place.get_policy());
}

fn main() {
    let mut taj = SecureHotel {
        name: "Taj Safe".to_string(),
    };
    let mut sasta_hostel = CheapHostel {
        name: "Zostel".to_string(),
    };

    // Taj ke paas dono traits hain, toh ye pass ho jayega.
    secure_booking(&mut taj, "Dhruv");
    check_status(&taj);

    // UNCOMMENT NEXT LINE TO SEE ERROR:
    // secure_booking(&mut sasta_hostel, "Sagar");
}

/*
EXPECTED OUTPUT:
--- Secure Booking System ---
Policy Found: 🛡️ Full Coverage Policy #123
🏨 Taj Safe booked for Dhruv
This place is certified: 🛡️ Full Coverage Policy #123
*/

/*
🧠 Deep Feel:
Multiple bounds `+` symbol se judte hain.
Ye filter ki tarah kaam karte hain—jo sab criteria fulfill karega wahi function ke andar aayega.
*/

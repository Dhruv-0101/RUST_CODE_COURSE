// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 27)
// ==============================

// Topic: The Clone Trait (Deep Copy)
// Feel: 
// "Bhai, Rust mein values 'move' ho jati hain (Transfer). 
// Agar aapko ek value ki 'Duplicate' (Hamshakal) chahiye, toh Clone karna padega. 
// Clone matlab: 'Nayi memory lo, aur sara data waha copy kar do'."

#[derive(Debug)]
struct Smartphone {
    model: String,
    owner: String,
}

// ==============================
// IMPLEMENTING CLONE
// ==============================
impl Clone for Smartphone {
    fn clone(&self) -> Self {
        // Feel: "Naya phone ban raha hai purane waale ka data dekh kar."
        println!("🔄 System: Cloning phone model '{}'...", self.model);
        
        Self {
            model: self.model.clone(), // String ko bhi clone karna padta hai
            owner: format!("{}'s Copy", self.owner), // Custom logic bhi add kar sakte hain!
        }
    }
}

fn main() {
    let my_phone = Smartphone {
        model: String::from("iPhone 15"),
        owner: String::from("Dhruv"),
    };

    // Bina clone ke, my_phone move ho jata.
    // Clone karne se humare paas DO phone ho gaye!
    let gift_phone = my_phone.clone();

    println!("--- Inventory ---");
    println!("Original: {:?}", my_phone);
    println!("Cloned:   {:?}", gift_phone);
}

/*
EXPECTED OUTPUT:
🔄 System: Cloning phone model 'iPhone 15'...
--- Inventory ---
Original: Smartphone { model: "iPhone 15", owner: "Dhruv" }
Cloned:   Smartphone { model: "iPhone 15", owner: "Dhruv's Copy" }
*/

/*
🧠 Deep Feel:
Clone humesha 'Explicit' hota hai. Aapko `.clone()` call karna hi padega.
Ye 'Expensive' ho sakta hai kyunki ye nayi memory allocate karta hai.
Standard types (String, Vec) already Clone implement karte hain.
Humne manual implement kiya taaki 'owner' field mein "Copy" word add kar sakein—derive se ye nahi hota!
*/


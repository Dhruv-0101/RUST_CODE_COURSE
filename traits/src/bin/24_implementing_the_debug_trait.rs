// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 24)
// ==============================

// Topic: Manually Implementing Debug Trait
// Feel: 
// "Bhai, humesha `#[derive(Debug)]` se kaam nahi chalta. 
// Kabhi-kabhi hume Debug output ko customize karna hota hai, 
// taaki logs mein hume extra information (ya formatted information) mile."

use std::fmt::{Debug, Formatter, Result};

struct SecretLaptop {
    brand: String,
    password: String, // Hum nahi chahte ki password debug logs mein dikhe!
}

// ==============================
// MANUAL DEBUG IMPLEMENTATION
// ==============================
impl Debug for SecretLaptop {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Hum password ki jagah [REDACTED] dikhayenge
        f.debug_struct("SecretLaptop")
            .field("brand", &self.brand)
            .field("password", &"******* [REDACTED] *******")
            .finish()
    }
}

fn main() {
    let mac = SecretLaptop {
        brand: String::from("Apple"),
        password: String::from("123456"),
    };

    println!("--- Debug View ---");
    // Notice: {:?} use karne pe humara manual logic chalega
    println!("{:?}", mac);
}

/*
EXPECTED OUTPUT:
--- Debug View ---
SecretLaptop { brand: "Apple", password: "******* [REDACTED] *******" }
*/

/*
🧠 Deep Feel:
Manual Debug implementation tab kaam aati hai jab:
1. Aap sensitive data (passwords, tokens) hide karna chahte ho.
2. Aapka struct complex hai aur aap use clean tarike se log karna chahte ho.
3. `f.debug_struct`, `f.debug_list`, `f.debug_map` jaise helper methods use karke 
aap standard standard look-and-feel maintain kar sakte ho.
*/
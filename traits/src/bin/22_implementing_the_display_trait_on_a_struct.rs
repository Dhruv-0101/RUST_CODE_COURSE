// 22,23,24,25 jane do
// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 22)
// ==============================

// Topic: Implementing Display Trait (User-friendly printing)
// Feel:
// "Bhai, Debug ({:?}) toh ghar ki baat hai, sirf developers dekhte hain.
// Lekin Display ({}) toh duniya (user) ko dikhana hai.
// Isme hum decide karte hain ki hamara Struct bahar ki duniya ko kaisa dikhega (Fancy and Clean)!"

use std::fmt::{Display, Formatter, Result};

struct Apple {
    kind: String,
    price: f64,
}

// ==============================
// IMPLEMENTING DISPLAY
// ==============================
impl Display for Apple {
    // fmt method mein asli magic hota hai
    fn fmt(&self, f: &mut Formatter) -> Result {
        // write! macro use karke hum output string banate hain
        // Feel: "Apple ko user ke liye sajake pesh kar rahe hain."
        write!(
            f,
            "🍎 Fresh {} Apple (Price: ${:.2})",
            self.kind, self.price
        )
    }
}

fn main() {
    let snack = Apple {
        kind: String::from("Himachali"),
        price: 1.50,
    };

    // Notice: Yaha humney {:?} nahi, sirf {} use kiya hai.
    // Kyunki Display implement ho chuka hai!
    println!("{}", snack);
}

/*
EXPECTED OUTPUT:
🍎 Fresh Himachali Apple (Price: $1.50)
*/

/*
🧠 Deep Feel:
Display trait implement karne ka matlab hai aapne apne struct ko 'Printable' bana diya.
Ab aap `.to_string()` bhi use kar sakte ho is struct pe!
Debug automatic 'derive' ho jata hai, par Display hume humesha MANUAL implement karna padta hai
kyunki Rust nahi janta ki aapka user kya dekhna chahta hai.
*/

// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 8)
// ==============================

// Topic: Methods accepting Closures (The Gatekeeper)
// Feel:
// "Bhai, socho aapne ek Tijori (Vault) banayi.
// Ye tijori kehti hai: 'Mujhe password tabhi dena jab main kahu,
// aur uske liye mujhe ek 'Procedure' (Closure) bhej do!'"

use std::io::stdin;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    // ------------------------------
    // THE UNLOCK METHOD 🔑
    // ------------------------------
    // Feel: `procedure` ek closure hai jo String return karega.
    // Humne `FnOnce` use kiya kyunki hume password sirf ek bar chahiye
    // aur tijori bhi open hone ke baad consume ho jayegi (`self`).
    fn unlock(self, procedure: impl FnOnce() -> String) -> Option<String> {
        // Robot procedure chala raha hai password lene ke liye
        let password = procedure();

        if password == self.password {
            println!("Robot: Password Match! Access Granted.");
            Some(self.treasure)
        } else {
            println!("Robot: WRONG! Alarm triggered 🚨");
            None
        }
    }
}

fn main() {
    let vault = Vault {
        password: String::from("topsecret"),
        treasure: String::from("Gold"),
    };

    // ------------------------------
    // THE HACKING CLOSURE 💻
    // ------------------------------
    // Ye closure user se input maangta hai.
    let hack = || {
        let mut user_input = String::new();
        println!("Robot: Enter password to crack the vault:");
        stdin().read_line(&mut user_input).expect("Failed to read");
        user_input.trim().to_string()
    };

    // Procedure manager ko closure de diya
    let extraction = vault.unlock(hack);

    println!("Bhai, result ye raha: {:?}", extraction);
}

/*
EXPECTED OUTPUT (If you type 'topsecret'):
Robot: Enter password to crack the vault:
topsecret
Robot: Password Match! Access Granted.
Bhai, result ye raha: Some("Gold")

EXPECTED OUTPUT (If you type wrong):
Robot: Enter password to crack the vault:
admin123
Robot: WRONG! Alarm triggered 🚨
Bhai, result ye raha: None
*/

/*
🧠 Deep Feel:
1. impl FnOnce() -> String: Iska matlab hai ki parameter koi bhi closure ho sakta hai
   jo string return kare.
2. Inversion of Control: `Vault` ko nahi pata password kaise milega,
   wo bas closure ko 'Call' karta hai. User decide karta hai hacking logic.
3. Consuming Self: `self` (without &) lene se tijori ek baar use hone ke baad
   destroy ho jati hai. Safety first!

Bhai, ye ek 'Smart Lock' hai jo aapse logic maang raha hai.
*/

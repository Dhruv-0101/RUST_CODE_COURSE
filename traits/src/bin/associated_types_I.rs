// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 33 - Associated Types I)
// ==============================

// Topic: Associated Types (The 'type' keyword inside Trait)
// Feel: 
// "Bhai, Generics (Trait<T>) aur Associated Types (type Item) mein kya farak hai? 
// Associated types tab use karo jab aap chahte ho ki ek struct ke liye 
// us trait ka sirf EK HI version ho. 
// Jaise 'Lunch' jab add hoga, toh result hamesha 'Lunch' hi aana chahiye!"

use std::ops::Add;

#[derive(Debug)]
struct Lunch {
    cost: f64,
}

// Implement Add Trait (Jo internally Associated Type 'Output' use karta hai)
impl Add for Lunch {
    // Ye hai 'Associated Type'
    // Feel: "Compiler ko bata rahe hain ki addition ka 'Result' kya hoga."
    type Output = Lunch;

    fn add(self, rhs: Self) -> Self::Output {
        Lunch {
            cost: self.cost + rhs.cost,
        }
    }
}

fn main() {
    let subway = Lunch { cost: 500.0 };
    let pizza = Lunch { cost: 800.0 };

    // '+' operator internally 'add' method call karta hai
    let total = subway + pizza;

    println!("--- Bill Report ---");
    println!("Total Lunch Cost: {:?}", total);
}

/*
EXPECTED OUTPUT:
--- Bill Report ---
Total Lunch Cost: Lunch { cost: 1300.0 }
*/

/*
🧠 Deep Feel:
Associated types code ko clean banate hain. 
Agar hum Generics use karte, toh `impl Add<Lunch> for Lunch` likhna padta, 
aur har jagah `Lunch` repeat karna padta. 
Associated types se hum kehte hain: 'Is implementation ke liye, ye type FIX hai'.
*/


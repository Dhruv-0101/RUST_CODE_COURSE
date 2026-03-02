// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 13)
// ==============================

// Topic: Trait Scope (Trait must be in scope to use its methods)
// Feel: 
// "Bhai, ye sabse bada GHOST ERROR hai. 
// Saara code sahi hai, Struct bhi wahi hai, lekin method nahi mil raha!
// Kyun? Kyunki aapne Trait ko 'use' karke room (scope) mein nahi bulaya."

use std::ops::Add; // Agar ye nahi hota, toh 'a.add(b)' kaam nahi karta.
use std::str::FromStr; // Agar ye nahi hota, toh 'u64::from_str' kaam nahi karta.

fn main() {
    // 1. Standard Addition using Trait
    let a: i32 = 5;
    let b: i32 = 10;
    
    // `.add()` method Trait 'Add' ke andar define hai.
    // Agar hum `use std::ops::Add;` upar nahi likhte, toh compiler error deta.
    let sum = a.add(b);
    println!("Sum is: {sum}");

    // 2. Parsing String using Trait
    // `from_str` method Trait 'FromStr' ke andar define hai.
    let numeric_count = u64::from_str("500");
    println!("Parsed Number: {}", numeric_count.unwrap());
}

/*
EXPECTED OUTPUT:
Sum is: 15
Parsed Number: 500
*/

/*
🧠 Deep Feel:
Traits Rust mein 'Extensions' ki tarah hain. 
Jab tak aap module ko 'use' nahi karte, uski powers (methods) 
aapke types (structs) pe activate nahi hoti. 
So, if a method is missing despite the type being right, ALWAYS check the Trait import!
*/


// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 34 - Associated Types II)
// ==============================

// Topic: Trait Bounds with Associated Types
// Feel: 
// "Bhai, agar aap Generic function bana rahe ho, toh aapko compiler ko batana padega 
// ki 'Output' type wahi honi chahiye jo hum expect kar rahe hain."

use std::ops::Add;

// Function jo kisi bhi type 'T' ko add kar sake
// Lekin shart (Bound) hai: T must implement Add, AUR uska Output type bhi 'T' hi hona chahiye!
fn universal_adder<T>(a: T, b: T) -> T 
where 
    T: Add<Output = T> 
{
    a + b
}

fn main() {
    let int_total = universal_adder(50, 100);
    let float_total = universal_adder(10.5, 20.5);

    println!("--- Universal Adder ---");
    println!("Int: {}, Float: {}", int_total, float_total);
}

/*
EXPECTED OUTPUT:
--- Universal Adder ---
Int: 150, Float: 31
*/

/*
🧠 Deep Feel:
`T: Add<Output = T>` ka matlab hai:
1. `T` addition support karta hai.
2. `T + T` karne par result `T` hi aata hai (Integer + Integer = Integer).
Agar hum sirf `T: Add` likhte, toh Rust ko pata nahi chalta ki return type kya hogi, 
kyunki do cheezon ko add karke result kuch teesra bhi ho sakta hai!
*/


// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 1)
// ==============================

// Topic: Concrete Lifetimes (Birth and Death of a Value)
// Feel: 
// "Bhai, Lifetimes koi scary chiz nahi hai. 
// Ye bas ye batati hai ki ek variable kab 'Paida' (Birth) hua aur kab 'Khatam' (Drop) hua.
// Isse 'Concrete Lifetime' kehte hain."

fn main() {
    // Feel: variable 'a' yaha paida hua (Start of Lifetime)
    let a = 10; 
    
    println!("Value of a is: {}", a);

    // Feel: variable 'a' yaha khatam ho jayega (End of Lifetime)
} // <--- Lifetime of 'a' ends here.

/*
EXPECTED OUTPUT:
Value of a is: 10
*/

/*
🧠 Deep Feel:
Lifetime ka matlab hai: Time period jab variable memory mein valid hai.
Jab tak 'a' is curly brace `{}` ke andar hai, wo zinda hai. 
Jaise hi brace khatam, memory free! 
Isse Rust 'Deterministic' banta hai (Humne pehle hi bata diya kab memory free hogi).
*/
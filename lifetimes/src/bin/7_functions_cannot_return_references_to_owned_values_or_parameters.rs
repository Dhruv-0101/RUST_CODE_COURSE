// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 7)
// ==============================

// Topic: Ghost References (Returning Local References)
// Feel: 
// "Bhai, sabka thikana scope ke andar hai. 
// Function ke andar joh variable paida hua (Locale), wo function ke khatam hone par mar jayega. 
// Aap function se uska 'Ghost' (Reference) return nahi kar sakte!"

/* 🛑 YE CODE COMPILE NAHI HOGA
fn create_number_reference(number: i32) -> &i32 {
    // number yaha paida (Parameter) ho gaya.
    &number // Feel: 'number' ka pata return kar raha hun. 
} // <--- 'number' yaha mar gaya! Return hone par pata kisi dead variable ka hoga.
*/

/* 🛑 INVALID RETURN
fn create() -> &i32 {
    let age = 32; 
    &age // Ghost reference! 'age' yaha mar jayega.
} 
*/

fn main() {
    println!("Bhai, function local variables ka reference return nahi kar sakte.");
    println!("Asli Solution: Reference ki jagah Pura 'Value' return kardo!");
}

/*
EXPECTED OUTPUT:
Bhai, function local variables ka reference return nahi kar sakte.
Asli Solution: Reference ki jagah Pura 'Value' return kardo!
*/

/*
🧠 Deep Feel:
Agar compiler allow karta, toh hum 'Dangling Pointer' ki duniya mein hote. 
Address valid hota lekin data 'Zombie' ban chuka hota (No memory control).
Rust's rule: 'Pata (Reference) sirf tab tak valid hai jab tak Data (Owner) zinda hai.'
Rule Simple: "Function ke andar se bahar sirf wo ja sakta hai joh khud Owner hai."
*/

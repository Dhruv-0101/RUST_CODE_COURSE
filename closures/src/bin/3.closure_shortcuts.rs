// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 2)
// ==============================

// Topic: Closure Shortcuts & Type Inference
// Feel:
// "Bhai, Closures 'Shortcuts' ke badshah hain.
// Rust kehta hai: 'Agar logic chota hai, toh na curly braces chahiye, na types!'
// Bas pipes |...| lagao aur kaam shuru!"

fn main() {
    let multiplier = 5;

    // ------------------------------
    // 1. THE ULTIMATE SHORTCUT 🏎️
    // ------------------------------
    // Feel: No curly braces `{}`, no `-> i32`, no explicit `i32` for value.
    // Compiler: "Bhai, chill! Main pehle use se guess kar lunga."
    let multiply_by = |value| value * multiplier;

    // Jab humne 3 diya, compiler ne decide kiya: "Accha, 'value' i32 hai!"
    println!("Shortest result: {}", multiply_by(3));

    // ------------------------------
    // 2. THE TYPE LOCK-IN 🔒
    // ------------------------------
    // Mirror closure: jo doge, wahi wapas milega.
    let mirror = |value| value;

    // Pehli baar String diya...
    println!("First mirror: {}", mirror("Hello"));

    /*
    🛑 THE 'AHA!' ERROR:
    Ek baar mirror ne "String" ke liye kaam kar liya, toh ab wo
    'i32' ke liye nahi chalega! Kyunki type lock ho gaya hai.

    // println!("{}", mirror(10)); // Error: expected `&str`, found `i32`
    */

    println!("Bhai, dhyan dena: Inference sirf PEHLI bar call pe hoti hai!");
}

/*
EXPECTED OUTPUT:
Shortest result: 15
First mirror: Hello
Bhai, dhyan dena: Inference sirf PEHLI bar call pe hoti hai!
*/

/*
🧠 Deep Feel:
1. One-liners: Agar ek hi line ka code hai, toh `{}` ki zaroorat nahi.
2. Inference: Types (`i32`, `&str`) likhna optional hai.
3. Consistency: Ek closure multiple types ke liye 'Generic' nahi banta automatically.
   Jo pehle call mein de diya, wahi uski pehchan ban gayi!
*/

// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 2)
// ==============================

// Topic: Inner Scopes and Explicit Drops
// Feel: 
// "Bhai, lifetime hamesha function ke khatam hone ka wait nahi karti.
// Hum variables ko 'Inner Scope' (Andar wale braces) mein qaid kar sakte hain 
// ya fir manually `drop()` karke unhe pehle hi khatam kar sakte hain."

fn main() {
    println!("--- Main Start ---");

    { // <--- Inner Scope Start
        let b = 50;
        println!("Value of b inside scope: {}", b);
    } // <--- 'b' ki lifetime yaha KHATAM! 
    
    // println!("{}", b); // 🛑 ERROR: 'b' ab exist nahi karta!

    let c = String::from("Rust Material");
    println!("Value of c: {}", c);

    drop(c); // <--- 'c' ki lifetime manually YAHI khatam kar di!
    
    // println!("{}", c); // 🛑 ERROR: 'c' ab invalidate ho chuka hai.

    println!("--- Main End ---");
}

/*
EXPECTED OUTPUT:
--- Main Start ---
Value of b inside scope: 50
Value of c: Rust Material
--- Main End ---
*/

/*
🧠 Deep Feel:
Lifetime fixed nahi hoti. 
Andar wale curly braces `{}` ek chota 'island' hai jisme 'b' paida hua aur wahi mar gaya.
`drop()` function compiler ko bolta hai: 'Iska kaam ho gaya, memory khali karo'. 
Lifetimes help Rust ensure ki hum kisi mare hue variable ko access na karein!
*/

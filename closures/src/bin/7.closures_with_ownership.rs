// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 5)
// ==============================

// Topic: Capturing Ownership (FnOnce Trait)
// Feel:
// "Bhai, ye closure 'Lalchi' (Greedy) hai.
// Ye variable ko sirf dekhta ya badalta nahi, ye use 'Apna' hi bana leta hai!
// Ek bar ownership Move ho gayi, toh bahar wali value khatam."

fn main() {
    // ------------------------------
    // 1. SIMPLE COPY DATA (i32)
    // ------------------------------
    let number = 13;
    let capture_number = || number; // yaha bas copy ho raha hai
    let a = capture_number();
    let b = capture_number();
    println!("i32 stays alive: {} {}", a, b);

    // ------------------------------
    // 2. OWNERSHIP DATA (String) - THE AHA! MOMENT 🎭
    // ------------------------------
    let first_name = String::from("Alice");

    // Feel: capture_string closure ke andar humne `person = first_name` kiya.
    // Iska matlab `first_name` closure ke andar 'MOVE' ho gaya!
    let capture_string = || {
        let person = first_name; // Ownership Move!
        println!("Robot owns: {}", person);
    };

    capture_string();

    /*
    🛑 THE ERROR LOGIC:
    1. `capture_string();` dobara nahi chal sakta!
       Kyunki pehli bar mein `first_name` consume ho gaya.
    2. Main function mein `first_name` use nahi ho sakta!
       Kyunki robot ne use 'EAT' (Own) kar liya hai.

    capture_string(); // Error: closure is `FnOnce`
    // println!("{}", first_name); // Error: value borrowed after move
    */
    capture_string(); // Error: closure is `FnOnce`

    println!("Bhai, dhyan dena: Ownership move matlab 'One-Time' power!");
}

/*
EXPECTED OUTPUT:
i32 stays alive: 13 13
Robot owns: Alice
Bhai, dhyan dena: Ownership move matlab 'One-Time' power!
*/

/*
🧠 Deep Feel:
1. Ownership: Agar closure variable ko 'Consume' (move out) karta hai, toh use `FnOnce` kehte hain.
2. The 'FnOnce' Trait: 'Once' matlab ye closure sirf ek hi bar call ho sakta hai.
   Kyunki usne jo data capture kiya tha, wo pehli bari mein hi destroy/move kar diya.
3. Why? Kyunki Rust memory safety ke liye ensure karta hai ki aap 'Gone' data ko access na karo.

Bhai, ye ek 'Suicide Mission' wala mini robot hai.
*/

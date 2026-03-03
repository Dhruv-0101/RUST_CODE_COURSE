// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 11)
// ==============================

// Topic: Passing Functions to Closure-Parameter Functions
// Feel:
// "Bhai, normal functions aur closures cousins hain.
// Jahan Rust ek closure mang raha ho (`Fn`, `FnMut`, etc.),
// wahan aap ek normal function bhi bhej sakte ho.
// Dono ka interface same hai toh Rust ko koi dikkat nahi!"

fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    println!("--- Recruiter is hiring a task ---");
    procedure();
    procedure();
    procedure();
}

// Ye ek shaanti se baitha normal function hai
fn bake_cake() {
    println!("🎂 Robot baking a simple cake...");
}

fn main() {
    // ------------------------------
    // 1. PASSING A NORMAL FUNCTION 🍰
    // ------------------------------
    // Feel: Hamne `bake_cake` ko variable ki tarah bhej diya.
    execute_thrice(bake_cake);

    // ------------------------------
    // 2. REAL-WORLD EXAMPLE: unwrap_or_else
    // ------------------------------
    let option: Option<Vec<String>> = None;

    // Feel: `unwrap_or_else` kehta hai: "Bhai, agar None hai toh koi method/closure do."
    // Hamne `Vec::new` jo ki ek function hai, wahi bhej diya!
    // No pipes `||`, no closure, just the raw function pointer.
    let collection = option.unwrap_or_else(Vec::new);

    println!("Final collection: {:?}", collection);
}

/*
EXPECTED OUTPUT:
--- Recruiter is hiring a task ---
🎂 Robot baking a simple cake...
🎂 Robot baking a simple cake...
🎂 Robot baking a simple cake...
Final collection: []
*/

/*
🧠 Deep Feel:
1. Compatibility: All functions (`fn`) implement `Fn`, `FnMut`, and `FnOnce` traits
   because they don't capture any environment! They are pure.
2. Short Syntax: `option.unwrap_or_else(Vec::new)` is much cleaner than
   `option.unwrap_or_else(|| Vec::new())`.
3. Power: Dono same hain, bas closure mein environment capture karne ki EXTRA power hoti hai.

Bhai, ye 'Interchangeable Parts' hai programmers ke toolkit mein.
*/

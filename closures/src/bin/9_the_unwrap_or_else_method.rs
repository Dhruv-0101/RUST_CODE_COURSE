// ===============================================
// RUST CLOSURES — THE 'UNWRAP_OR_ELSE' FEEL (EPISODE 9)
// ===============================================

// Topic: The unwrap_or_else Method
// Feel:
// "Bhai, ye 'Smart Backup Plan' hai.
// Difference samjho:
// 1. unwrap_or (Dumb): Backup Generator hamesha chalu hai, paise (CPU) kharch ho rahe hain.
// 2. unwrap_or_else (Smart): Generator tabhi chalega jab bijli (Some value) jayegi!"

fn main() {
    println!("--- The 'unwrap_or_else' Experience ---");

    // ---------------------------------------------------------
    // 🍕 SCENARIO 1: POSITIVE CASE (Value is SOME)
    // ---------------------------------------------------------
    let food_box = Some("Paneer Tikka");

    // Feel: Food box mein pehle se Paneer hai, toh humein closure disturb
    // karne ki zaroorat nahi padegi.
    let dish = food_box.unwrap_or_else(|| {
        // Ye code kabhi run hi nahi hoga!
        // Rust dekhega: "Bhai, Paneer toh hai, kyun bewajah pizza banwayu?"
        println!("🚨 ALERT: Robot is wasting time cooking Pizza...");
        "Pizza"
    });

    println!("Table par kya hai? {}", dish); // Output: Paneer Tikka

    // ---------------------------------------------------------
    // 🏜️ SCENARIO 2: EMPTY CASE (Value is NONE)
    // ---------------------------------------------------------
    let empty_box: Option<&str> = None;

    // Feel: Ab box khali hai. Ab backup generator (Closure) ko
    // trigger hona hi padega.
    let fallback_dish = empty_box.unwrap_or_else(|| {
        println!("💡 LOGIC: Box was empty, let's trigger the backup plan...");
        "Masala Dosa"
    });

    println!("Fallback Dish: {}", fallback_dish);
}

/*
🧠 DEEPER FEEL:

1. LAZY EVALUATION (Alsi Robot):
   - `unwrap_or_else` ek 'Alsi (Lazy) Robot' ki tarah hai. Jab tak use
     value mil rahi hai, wo chup rehta hai.
   - Jab use 'None' dikhta hai, tabhi wo closure ke andar ghusta hai.
   - Isse 'Performance' bachti hai, kyunki unnecessary code nahi chalta.

2. UNWRAP_OR vs UNWRAP_OR_ELSE (Dumb vs Smart):

   - `option.unwrap_or(calculate_value())`:
     Yaha `calculate_value()` hamesha chalega, chahe option `Some` ho ya `None`.
     (Dumb Backup - Paisa waste!)

   - `option.unwrap_or_else(|| calculate_value())`:
     Yaha `calculate_value()` sirf TABHI chalega jab option `None` ho.
     (Smart Backup - Paisa vasool!)

3. USE CASE:
   - Agar default value nikalne ke liye Database query karni pad rahi hai
     ya koi badi calculation hai, toh humesha `unwrap_or_else` use karo!

Summary:
- unwrap_or = Simple values ke liye (like "Default", 0, false).
- unwrap_or_else = Logic aur calculations ke liye (Lazy execution).
*/

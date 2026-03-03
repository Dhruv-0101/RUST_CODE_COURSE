// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 11)
// ==============================

// Topic: Lifetime Elision Rule 1 (The Single Parent)
// Feel: 
// "Bhai, Rust hamesha `'a` likhwate nahi thakta. 
// Usne banaye hain 'Elision Rules' (Shortcut Rules). 
// Rule 1: 'Agar function mein EK HI reference input ayega, toh 
// return wala reference bhi USI ki lifetime pe depend karega!'"

// Feel: Yaha humne `'a` nahi likha, par compiler khud likh deta hai.
fn first_two(items: &[String]) -> &[String] {
    // Rust auto assumes: fn first_two<'a>(items: &'a [String]) -> &'a [String]
    &items[..2]
}

fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];

    let result = first_two(&cities);
    println!("Selected: {:?}", result);
}

/*
EXPECTED OUTPUT:
Selected: ["London", "New York"]
*/

/*
🧠 Deep Feel:
Compiler smart hai. Use pata hai ki agar 1 data item input mein hai, 
toh result usi data ka hi koi part hoga. Isliye use `'a` ki tension 
hume dene ki zaroorat nahi hoti. 
Rule 1 summary: 1 input reference = All output references get same lifetime. 
*/

// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 12)
// ==============================

// Topic: Lifetime Elision Rule 2 (Multiple Parents - Confusion!)
// Feel: 
// "Bhai, Rule 2 ye hai ki: 'Agar 1 se zyada input references hain, 
// toh compiler confused ho jata hai ki result kispe depend karega?'. 
// Tab hume `'a` likhna hi padta hai (No automatic elision)!"

// 🛑 ERROR WITHOUT 'a
/*
fn pick_one(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}
*/

// Feel: Compiler ko bolna pada: 'Bhai, s1 aur s2, dono same 
// time tak zinda rehne chahiye, aur return bhi utna hi zinda rahega.'
fn pick_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn main() {
    let name1 = String::from("Dhruv");
    let name2 = String::from("Antigravity");

    let result = pick_longest(&name1, &name2);
    println!("Longest: {}", result);

    println!("Bhai, dhyan rakhna: Rule 2 kehti hai ki multiple 
    input references ke liye manual intervention chahiye (Explicit 'a)!");

}

/*
EXPECTED OUTPUT:
Longest: Antigravity
Bhai, dhyan rakhna: Rule 2 kehti hai ki multiple 
    input references ke liye manual intervention chahiye (Explicit 'a)!
*/

/*
🧠 Deep Feel:
Agar compiler allow karta, toh use pata nahi hota ki agar 's1' 
pehle mar jaye aur 's2' abhi ho, toh kya result valid hai? 
Isliye multiple inputs par Rust safer side mein rehta hai 
aur aapke logic ke hisab se `'a` maangta hai. 
Manual lifetimes = Full clarity.
*/

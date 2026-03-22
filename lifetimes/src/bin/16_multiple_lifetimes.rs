// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 16)
// ==============================

// Topic: Multiple Lifetimes ('a, 'b) — Independent Scopes
// Feel:
// "Bhai, multiple lifetimes humein 'Aazadi' (Freedom) deti hain.
// Jab do references alag-alag thikano se aa rahe hon aur unka time span
// ek-dusre se alag ho, toh hum unhe ek hi 'a tag mein nahi bandh sakte!"

/*
  🖊️ ANALOGY (The "Friend & Airbnb" Story):
  Dost A (Mumbai) ek mahine ke liye rukne aaya hai. 🏨
  Dost B (Delhi) sirf ek din ke liye Airbnb mein ruka hai. ⏳

  Agar hum kahenge: "Bhai, dono ko ek hi 'a lifetime dedo",
  toh Rust sochega: 'Dono ek hi bundle hain'.
  Rust sabse CHOTE wale (Dost B - 1 din) ko pakad ke baithega!
  Toh aap Dost A se borrow ki hui cheez bhi 1 din ke baad use nahi kar paoge. 🚫

  Lekin agar aap '<'a, 'b>' doge, toh aap compiler ko bata rahe ho:
  'Bhai, inka time-span alag-alag hai. Tension mat lo!' ✅
*/

// Function with multiple independent lifetimes
fn get_starting_point<'a, 'b>(from: &'a str, _to: &'b str) -> &'a str {
    // Return value sirf 'a (from) pe depend karti hai.
    // '_to' chahe kab bhi mar jaye, humein farak nahi padta!
    from
}

fn main() {
    let city1 = String::from("Mumbai (Dost A - Long Stay)"); // Live until end of main
    let result;

    {
        let city2 = String::from("Delhi (Dost B - Short Stay)"); // inner scope (Dies at })

        // 🧪 MAGIC HERE:
        // 'city1' ki lifetime 'a hai (Badi wali)
        // 'city2' ki lifetime 'b hai (Choti wali)
        // Hamara 'result' sirf city1 ('a) se juda hua hai.
        result = get_starting_point(&city1, &city2);

        println!("Inner: result is pointing to {}", result);
    } // <--- city2 (Dost B) yaha mar gaya! 💀

    // 🎯 POINT:
    // Agar humne '<'a>' ek hi tag use kiya hota dono ke liye,
    // toh yaha 'result' use karne par ERROR aata!
    // Kyun? Kyunki compiler sabse choti (city2) ki lifetime 'result' pe force kar deta.

    // Lekin '<'a, 'b>' ki wajah se 'result' abhi bhi valid hai!
    println!("Outer: Mumbai is still alive: {}", result);

    println!("Bhai, dhyan dena: Multiple lifetimes = Accuracy and Freedom! 🦾✨");
}

/*
EXPECTED OUTPUT:
Inner: result is pointing to Mumbai (Dost A - Long Stay)
Outer: Mumbai is still alive: Mumbai (Dost A - Long Stay)
Bhai, dhyan dena: Multiple lifetimes = Accuracy and Freedom! 🦾✨
*/

/*
🧠 Deep Feel:
1. One Lifetime Tag ('a): "Bhai, sab ek hi kasti (boat) mein sawar hain."
   (If one drowns, everyone drowns for the return value).

2. Multiple Lifetime Tags ('a, 'b): "Bhai, tumhari kasti alag, meri kasti alag."
   (Independent survival! One can die, while the other lives on).

Rule: 'impl Trait' ya simple references mein hum tab multiple use karte hain
jab return value sirf EK input se connected ho aur dusra input jaldi 'expire' ho raha ho.
*/

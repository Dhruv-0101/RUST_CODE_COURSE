// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 30 - Eq)
// ==============================

// Topic: Eq (Total Equality)
// Feel: 
// "Bhai, PartialEq toh theek hai, lekin 'Eq' kya hai? 
// Eq ek 'Marker' trait hai jiska koi method nahi hota. 
// Ye sirf compiler ko batata hai ki: 'Ye type humesha khud ke barabar (reflexive) hogi'."

// Note: f64 (Floats) Eq implement nahi karte kyunki NaN != NaN hota hai!
#[derive(PartialEq, Eq, Debug)]
struct Flight {
    id: u32,
    destination: String,
}

fn main() {
    let f1 = Flight { id: 101, destination: "Delhi".to_string() };
    
    // Eq ki wajah se hum guarantee dete hain ki f1 hamesha f1 ke barabar hoga.
    println!("Is f1 == f1? {}", f1 == f1);

    // Integers Eq implement karte hain
    let a: i32 = 10;
    println!("i32 comparison: {} == {} is {}", a, a, a == a);

    // Floats implement PartialEq ONLY (Not Eq)
    let nan = f64::NAN;
    println!("--- The Float Exception ---");
    println!("Is NaN == NaN? {}", nan == nan); // Output: false! 
    // Isiliye floats 'Eq' nahi hote, sirf 'PartialEq' hote hain.
}

/*
EXPECTED OUTPUT:
Is f1 == f1? true
i32 comparison: 10 == 10 is true
--- The Float Exception ---
Is NaN == NaN? false
*/

/*
🧠 Deep Feel:
Eq matlab Total Equality. 
Iska matlab hai: For all `a`, `a == a` must be true.
Floating point numbers (f32, f64) mein `NaN` (Not a Number) khud ke barabar nahi hota, 
isiliye floats sirf `PartialEq` hote hain. 
Agar aap kisi struct mein float rakhte ho, toh aap `Eq` derive nahi kar sakte!
*/


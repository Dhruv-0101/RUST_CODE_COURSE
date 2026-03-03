// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 0)
// ==============================

// Topic: Why Closures? (Normal Functions vs Closures)
// Feel:
// "Bhai, normal functions thode 'Ziddi' hote hain.
// Agar aapne unhe kisi variable ke scope ke andar banaya,
// tab bhi wo us variable ko 'Dekh' nahi sakte.
// Is problem ko solve karne ke liye Closures aaye!"

fn main() {
    let multiplier = 3;

    // ------------------------------
    // 1. NESTED FUNCTION (The Blind One) 🙈
    // ------------------------------
    fn multiply_by_fn(value: i32) -> i32 {
        // 🛑 Error: can't capture dynamic environment in a function item
        // multiply_by_fn kehta hai: "Bhai, ye 'multiplier' kya hai? Main nahi janta!"
        // value * multiplier
        value * 2 // Majboori mein humne 2 likh diya kyunki wo bahar nahi dekh sakta
    }

    println!("Nested Function result: {}", multiply_by_fn(5));

    // ------------------------------
    // 2. THE SMART CLOSURE (The All-Seeing 👁️)
    // ------------------------------
    // Feel: "Ab syntax ka post-mortem karte hain:"
    // 1. '|value|' -> Pipes ke andar parameters aate hain (jaise (value: i32) function mein hota hai).
    // 2. 'value * multiplier' -> Ye body hai. Ek line hai toh na {} chahiye, na return keyword!
    // 3. 'multiplier' -> Ye bahar se 'Chura' (Capture) liya gaya hai.
    let multiply_by_closure = |value| value * multiplier;

    println!("Closure result: {}", multiply_by_closure(5));

    /*
    🧠 SYNTAX BREAKDOWN:
    Bhai, syntax ko aise dekho:

    let closure_name = | param1, param2 | { body };

    Normal Function:   fn  name ( param1: i32 ) -> i32 { return body; }
    Closure (Full):    let name = | param1: i32 | -> i32 { return body; };
    Closure (Short):   let name = | param1 | body;

    Asli Feel: Pipes `| |` ek darwaza (Gate) hai parameters ke liye.
    Jaise hi parameter darwaze se andar aaya, body ne bahar ke variables (multiplier)
    ke saath milkar kaam tamam kar diya!
    */
}

/*
EXPECTED OUTPUT:
Nested Function result: 10
Closure result: 15
*/

/*
🧠 Deep Feel:
1. Lexical Scope: Closures jaha define hote hain, waha ke environment ko 'Enclose' karte hain
   (isiliye unhe 'Closures' kehta hain).
2. Function Limitations: `fn` hamesha static hote hain. Unhe sirf apne parameters
   aur global constants se matlab hota hai.
3. Flexibility: Code ko dynamic banaye ke liye Closures ka use karo.

Bhai, ye 'Environment' capture karne ki power hi Closures ko itna special banati hai.
*/

// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 4)
// ==============================

// Topic: Capturing Mutable References (FnMut Trait)
// Feel:
// "Bhai, ye closure ek 'Painter' ki tarah hai.
// Ye sirf dekhta nahi, ye cheezon ko 'Badal' (Mutate) deta hai!
// Isse variable ka 'Exclusive' access chahiye (Mutable Borrow)."

fn main() {
    let mut numbers = vec![4, 8, 15, 16, 23, 42];

    // ------------------------------
    // THE MUTATING CLOSURE ✍️
    // ------------------------------
    // Feel: Closure ke variable `add_number` ko bhi `mut` hona padega!
    // Kyunki har baar ye call hoga, iska 'Internal State' ya data change hoga.
    let mut add_number = || {
        println!("Robot is adding 100...");
        numbers.push(100);
    };

    // add_number(); // Pehla 100
    // add_number(); // Dusra 100

    // 🛑 DHYAN DENA:
    // Agar main yaha `println!("{:?}", numbers);` karu, toh ERROR aayega!
    // Kyunki `add_number` ne pure 'numbers' ko 'Strictly Capture' kiya hua hai.
    // Jab tak robot apna kaam khatam nahi karta, main use nahi kar sakta.

    add_number();
    add_number();

    // ------------------------------
    // Kaam khatam, ab owner (main) use kar sakta hai
    // ------------------------------
    println!("Final list: {:?}", numbers);
}

/*
EXPECTED OUTPUT:
Robot is adding 100...
Robot is adding 100...
Final list: [4, 8, 15, 16, 23, 42, 100, 100]
*/

/*
🧠 Deep Feel:
1. Mutability: Closure ne `&mut numbers` liya hai.
2. Exclusion: Jab tak `add_number` zinda hai aur use ho raha hai,
   aap original `numbers` ko read ya write nahi kar sakte. (Borrow Checker 🛡️)
3. The 'FnMut' Trait: Aise closures jo bahar ke data ko modify karte hain,
   wo `FnMut` implement karte hain. Inhe multiple times call kiya ja sakta hai.

Bhai, ye ek 'Worker' mini robot hai jo data update kar raha hai.
*/

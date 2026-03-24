// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 3)
// ==============================

// Topic: Capturing Immutable References (Fn Trait)
// Feel:
// "Bhai, jab closure bahar ki duniya ko sirf 'Dekhna' chahta hai,
// toh wo un variables ko 'Immutably Borrow' kar leta hai.
// Ye sabse shareef closure hai, sabke saath mil-baat kar rehta hai!"

fn main() {
    // Ek vector jo main data hai
    let numbers = vec![4, 8, 15, 16, 23, 42];

    // ------------------------------
    // THE OBSERVER CLOSURE 👀
    // ------------------------------
    // Feel: Empty pipes `||` matlab koi parameters nahi chahiye.
    // Ye closure 'numbers' ko capture kar raha hai sirf read karne ke liye.
    let print_numbers = || println!("Closure sees: {:?}", numbers);

    // Bahar wala main function bhi dekh sakta hai (Multiple readers allowed!)
    println!("Main sees: {:?}", numbers);

    // Closure apna kaam karega
    print_numbers();
    print_numbers();

    // End mein bhi data safe hai
    println!("Main still sees: {:?}", numbers);
}

/*
EXPECTED OUTPUT:
Main sees: [4, 8, 15, 16, 23, 42]
Closure sees: [4, 8, 15, 16, 23, 42]
Closure sees: [4, 8, 15, 16, 23, 42]
Main still sees: [4, 8, 15, 16, 23, 42]
*/

/*
🧠 Deep Feel:
1. Immutability: Closure ne yaha `&numbers` liya hai.
2. Sharing: Kyunki ye sirf read kar raha hai, bahar wala code bhi
   read kar sakta hai (No violation of Borrowing rules).
3. The 'Fn' Trait: Aise closures jo sirf read karte hain (ya variable use nahi karte),
   wo `Fn` trait implement karte hain. Inhe kitni bhi bar call kiya ja sakta hai!

Bhai, ye ek basic 'Read-Only' mini robot hai.
*/

// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 6)
// ==============================

// Topic: The 'move' Keyword
// Feel:
// "Bhai, 'move' matlab 'Kidnapping'.
// Aap compiler ko order dete ho: 'Suno, ye variables sirf borrow mat karo,
// unhe poora ka poora closure ke andar kidnap (Move) karlo!'"

fn main() {
    let first_name = String::from("Alice");
    let last_name = String::from("Wonder");

    // ------------------------------
    // THE KIDNAPPER CLOSURE 🏃‍♂️💨
    // ------------------------------
    // Feel: Bina `move` ke ye closure sirf `&first_name` leta.
    // Par `move` lagane se ye variables hamesha ke liye closure ke ho gaye.
    let capture_string = move || {
        // Look! Ye variables closure ke 'Niji Property' (Owned) ban gaye hain.
        println!("Robot kidnapped: {} {}", first_name, last_name);
    };

    capture_string();
    capture_string(); // Ye multiple times chal raha hai? YES! 

    // Kyun? Kyunki isne variables ko capture (own) toh kiya,
    // par unhe 'Destroy' (Consume) nahi kiya. Bas print kiya.
    // Isliye ye `Fn` ban gaya (Multiple uses allowed).

    /* 🛑 THE MOVE ERROR:
    println!("{}", first_name); // Error: value borrowed after move!
    // Bhai, kidnapped insaan bahar thodi milega!
    */

    println!("Bhai, 'move' keyword thread-level programs mein bohot kaam aata hai.");
}

/*
EXPECTED OUTPUT:
Robot kidnapped: Alice Wonder
Robot kidnapped: Alice Wonder
Bhai, dhyan dena: 'move' keyword thread-level programs mein bohot kaam aata hai.
*/

/*
🧠 Deep Feel:
1. move vs ownership: Bina `move` ke, Rust guess karta hai (Read/Write/Consume).
   `move` lagane se aap Rust ko force karte ho ki 'Zabardasti Ownership lele'.
2. Iteration: `move` lagane ke baad bhi closure `Fn` ya `FnMut` ho sakta hai
   agar wo variables ko destroy nahi kar raha.
3. Why? Most common use: Threading. Jab aap variable ko dusre thread mein bhejte ho,
   toh original thread khatam ho sakta hai, isliye 'Ownership' bhejni zaroori hai.

Bhai, ye ek 'Transporter' mini robot hai jo saman shift kar raha hai.
*/

// ==========================================
// RUST CLOSURES — THE 'MOVE' FEEL (EPISODE 6)
// ==========================================

// Topic: The 'move' Keyword
// Feel:
// "Bhai, 'move' matlab 'No more Dosti, only Malliki' (No more friendship, only Ownership).
// Default mein closure sirf 'Borrow' karta hai, par `move` lagane se wo
// variables ko closure ki 'Niji Property' (Private Property) bana deta hai."

fn main() {
    println!("--- The 'move' Keyword Experience ---");

    let status = String::from("Bhai, main yaha hoon!");

    // ---------------------------------------------------------
    // 🛑 SCENARIO: WITHOUT 'move' (The Default Borrower)
    // ---------------------------------------------------------
    // let check_status = || println!("Status: {}", status);
    // check_status();
    // println!("Is status still alive? {}", status); // YES! (Kyunki borrow hua tha)

    // ---------------------------------------------------------
    // 🏃‍♂️💨 SCENARIO: WITH 'move' (The Kidnapper/Owner)
    // ---------------------------------------------------------
    // Feel: Jab hum `move` likhte hain, hum Rust compiler ko kehte hain:
    // "Bhai, is variable ko borrow mat kar, seedha closure ki pockets mein daal de."
    let capture_status = move || {
        // Look! Ye `status` ab closure ke andar settle ho gaya hai.
        println!("Closure inside: {}", status);
    };

    capture_status();

    // ---------------------------------------------------------
    // 🧠 THE 'MOVE' POST-MORTEM (Kyoon aur Kaise?)
    // ---------------------------------------------------------

    // 1. WHY THE ERROR BELOW?
    // println!("Main function says: {}", status); // 🚨 ERROR!
    // Feel: "Bhai, jo cheez move ho gayi, wo main() mein kaise mil sakti hai?"
    // `status` ab closure ka maalik hai, main() ne haqq kho diya hai.

    // 2. IS IT ALWAYS FnOnce?
    // NO! Look at this carefully. closure `capture_status` ko hum dobara bhi
    // call kar sakte hain:
    capture_status();
    // "Bhai, wait! Agar ownership move hui toh dobara call kaise?"
    // Reality: Kyunki closure ne `status` ko consume (destroy) nahi kiya,
    // bas use 'rakha' hua hai. Isliye ye `Fn` hi raha, bas ownership change hui.

    println!("\nBhai, niche ke notes mein asli feel hai!");
}

/*
🧠 DEEPER FEEL:

1. THE THREADING ANGLE (Dosti vs Safar):
   - Jab aap ek naya thread (Safar) shuru karte ho, toh original thread
     shanti se 'khatam' ho sakta hai.
   - Agar aapne sirf variables ka 'reference' bhej diya (Dosti), aur original
     thread (Dost) mar gaya, toh reference kachra (Dangling) ho jayega.
   - `move` isi liye hota hai ki variable poora ka poora 'Naye Ghar' (Closure)
     mein chala jaye, taaki naya thread azadi se chale.

2. MOVE = FORCED OWNERSHIP:
   - Rust normal closures mein khud guess karta hai (Borrow, MutBorrow, Move).
   - `move` lagane se aap Rust ko bolte ho: "Bhai, guess mat kar, FORCE MOVE kar."

3. COPY TYPES (The Exception):
   - Agar ye `String` ki jagah `i32` (Integer) hota, toh move nahi 'COPIED' hota.
   - Kyunki chote numbers copy ho jate hain, toh original variable bhi zinda rehta.

Summary:
- move = "Variable ab mera (Closure ka) hai!"
- Without move = "Bhai, variable thodi der ke liye de de, baad mein lautaa dunga."
*/

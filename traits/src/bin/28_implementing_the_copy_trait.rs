// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 28)
// ==============================

// Topic: The Copy Trait (Automatic Shallow Copy)
// Feel: 
// "Bhai, Clone toh manual tha (.clone() likhna padta tha). 
// Lekin 'Copy' bohot tez aur automatic hai. 
// Ye sirf un cheezon pe chalta hai jo Stack pe rehti hain (integer, float, fixed-size structs).
// Copy matlab: 'Bas bits copy kar do, puraane wale ko move mat karo'."

#[derive(Debug, Clone, Copy)] // Requirement: Copy ke liye Clone implement hona chahiye!
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Position { x: 10, y: 20 };
    
    // Yaha 'Copy' trait magic kar raha hai!
    // p1 move nahi hua, balki automatic ek humshakal (copy) ban gayi p2 ke liye.
    let p2 = p1; 

    println!("--- Coordinate Report ---");
    // Dono print ho jayenge, kyunki p1 abhi bhi zinda hai!
    println!("Position 1: {:?}", p1);
    println!("Position 2: {:?}", p2);
}

/*
EXPECTED OUTPUT:
--- Coordinate Report ---
Position 1: Position { x: 10, y: 20 }
Position 2: Position { x: 10, y: 20 }
*/

/*
🧠 Deep Feel:
1. `Copy` is implicit (automatic): Aapko `.copy()` nahi likhna padta, bas assignment `=` se ho jata hai.
2. `Copy` works only for Stack data: Agar kisi struct mein `String` or `Vec` hai, toh wo Copy nahi ho sakta!
3. All fields must be Copy: Agar struct ke saare fields Copy hain, tabhi struct Copy ban sakta hai.
Copy trait compiler ko ye batata hai: 'Ye data itna simple hai ki isse copy karna safe hai, move karne ki zaroorat nahi'.
*/

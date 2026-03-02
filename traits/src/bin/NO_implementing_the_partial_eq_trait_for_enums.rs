// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 29 - PartialEq)
// ==============================

// Topic: PartialEq (Equality Trait)
// Feel: 
// "Bhai, do cheezon ko compare kaise karein? 
// 'PartialEq' hume operator overloading deta hai (== aur !=). 
// Isse hum compiler ko batate hain ki do Musicians kab 'Equal' mane jayenge."

#[derive(Debug)]
enum Musician {
    SingerSongwriter(String),
    Band(u32),
}

// ==============================
// IMPLEMENTING PARTIALEQ
// ==============================
impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        // Feel: "Match karke dekho, kya dono same type aur same value ke hain?"
        match (self, other) {
            (Musician::SingerSongwriter(n1), Musician::SingerSongwriter(n2)) => n1 == n2,
            (Musician::Band(m1), Musician::Band(m2)) => m1 == m2,
            _ => false, // Agar ek Solo hai aur ek Band, toh kabhi barabar nahi ho sakte!
        }
    }
}

fn main() {
    let m1 = Musician::SingerSongwriter(String::from("Arijit"));
    let m2 = Musician::SingerSongwriter(String::from("Arijit"));
    let m3 = Musician::Band(4);

    println!("--- Music Comparison ---");
    // '==' trigger karega 'eq' method ko
    println!("Is m1 == m2? {}", m1 == m2); 
    println!("Is m1 == m3? {}", m1 == m3);
}

/*
EXPECTED OUTPUT:
--- Music Comparison ---
Is m1 == m2? true
Is m1 == m3? false
*/

/*
🧠 Deep Feel:
PartialEq manual implement tab karo jab aapko custom logic chahiye. 
Humesha yaad rakho, `a == b` internally `a.eq(&b)` call karta hai.
Most of the time `#[derive(PartialEq)]` se kaam chal jata hai, 
lekin manual implementation aapko full control deti hai (jaise case-insensitive comparison).
*/


// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 3)
// ==============================

// Topic: Lifetimes for References (The "Guest" Principle)
// Feel: 
// "Bhai, jab hum ek reference `&` banate hain, toh wo ek 'Mehmani' (Guest) ki tarah hota hai.
// Shart (Condition) ek hi hai: 'Mehmani' tabhi tak chal sakti hai jab tak 'Mezban' (Owner) zinda hai!"

fn main() {
    // Mezban (Owner) 'dog' yaha paida hua
    let dog = String::from("Watson");

    {
        // Reference 'my_pet' ek Mehman hai.
        // Iska lifetime Mezban ke lifetime ke ANDAR hi hona chahiye.
        let my_pet = &dog; 
        println!("Barking: {}", my_pet);
    } // <--- Reference 'my_pet' ki lifetime yaha khatam.

    println!("Owner is still alive: {}", dog);

} // <--- Mezban 'dog' yaha khatam.

/*
EXPECTED OUTPUT:
Barking: Watson
Owner is still alive: Watson
*/

/*
🧠 Deep Feel:
Reference ki apni ek separate lifetime hoti hai. 
Rust check karta hai ki: `Reference Lifetime <= Owner Lifetime`.
Agar reference owner ke marne ke baad bhi zinda rehne ki koshish kare, 
toh Rust usse 'Dangling Reference' bolta hai aur error deta hai. 
References hamesha kisi 'Concrete' value se bandhe hote hain.
*/

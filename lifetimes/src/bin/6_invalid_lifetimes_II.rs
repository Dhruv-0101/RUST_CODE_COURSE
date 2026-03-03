// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 6)
// ==============================

// Topic: Invalid Lifetimes II (The "Borrowed Key" Problem)
// Feel: 
// "Bhai, socho aapke paas ghar (cities) ki chaabi hai (favorite_cities). 
// Par aapne ghar kisi aur ko BECH diya (Move to places)! 
// Toh purani chaabi bekaar ho gayi, na? Purane ghar mein toh aap ja nahi sakte."

fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];

    // Reference (Chaabi) banayi
    let favorite_cities = &cities[0..2];

    /* 🛑 ERROR: Move happened!
    let places = cities; // Cities yaha moves (owner change) ho gaya!
    
    // Purana Reference INVALID ho gaya!
    // println!("{:?}", favorite_cities); 
    */

    println!("Cities: {:?}", favorite_cities);
    println!("Feel: Agar mein 'Favorite Cities' use karne se pehle 'Cities' ko 'Move' kar deta,");
    println!("toh Rust bolta: 'Bhai, chaabi purane owner ki hai, aur tumne naya owner bana diya!'");

}

/*
EXPECTED OUTPUT:
Cities: ["London", "New York"]
Feel: Agar mein 'Favorite Cities' use karne se pehle 'Cities' ko 'Move' kar deta,
toh Rust bolta: 'Bhai, chaabi purane owner ki hai, aur tumne naya owner bana diya!'
*/

/*
🧠 Deep Feel:
Movement (Ownership change) purane references ko 'Invalidate' kar deta hai.
Kyunki 'cities' apna ownership kisi ko de chuka hai (ya drop ho sakta hai), 
Rust ki memory safety guarantee dalti hai: 'Reference tab tak hi chalega jab tak Owner ko kisi ne chhawa na ho!'
*/

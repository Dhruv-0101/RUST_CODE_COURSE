// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 10)
// ==============================

// Topic: Lifetimes and Referents
// Feel: 
// "Bhai, 'Referent' matlab wo asiliyat (Data) jise reference point kar raha hai. 
// Rust ka suhana rule: 'Referent' hamesha 'Reference' se zyada ya barabar zinda rehna chahiye. 
// Agar Referent pehle mar gaya, toh Reference 'Anath' (Orphan/Dangling) ho jayega!"

fn select_first_two<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}

fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];

    // Case 1: cities (Referent) main scope mein hai.
    // two_cities (Reference) bhi wahi hai. Sab khush hain!
    let two_cities = {
        let cities_ref = &cities;
        select_first_two(cities_ref)
    };
    println!("Two Cities: {:?}", two_cities);

    {
        // Case 2: coffees (Referent) inner scope mein hai.
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let two_coffees = select_first_two(&coffees);
        println!("Two Coffees: {:?}", two_coffees);
    } // <--- coffees mar gaya. Iske baad 'two_coffees' use nahi ho sakta!

    println!("Main still running...");
}

/*
EXPECTED OUTPUT:
Two Cities: ["London", "New York"]
Two Coffees: ["Latte", "Mocha"]
Main still running...
*/

/*
🧠 Deep Feel:
Terminology Alert! 
- Reference: pointer `&`.
- Referent: Woh data jise point kiya ja raha hai (e.g. `cities`).
Rust bas ye check karta hai ki pointer ki lifetime kabhi bhi data ki lifetime 
se 'bahar' na nikal jaye. 
Is code mein `select_first_two` ke return value ki lifetime `'a` wahi hai 
jo input `items` ki hai. Simple and safe.
*/

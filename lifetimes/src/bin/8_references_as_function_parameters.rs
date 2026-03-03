// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 8)
// ==============================

// Topic: References as Parameters (The "Connector")
// Feel:
// "Bhai, agar aap ek function se reference return kar rahe ho,
// toh compiler puchta hai: 'Ye reference kaha se aaya?'.
// Agar ye input parameter se aaya hai, toh input wala variable
// bahar wali value se zyada zinda rehna chahiye!"

fn select_first_two(items: &[String]) -> &[String] {
    // Feel: Hum original 'items' ka hi ek tukda (slice) bhej rahe hain.
    // Iska thikana 'items' par hi depend karta hai.
    &items[..2]
}

fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];

    // cities zinda hai, toh two_cities valid hai.
    let two_cities = select_first_two(&cities);
    println!("First two: {:?}", two_cities);

    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        // 'coffees' scope ke andar hai, 'two_coffees' bhi scope tak hi chalega.
        let two_coffees = select_first_two(&coffees);
        println!("Coffees: {:?}", two_coffees);
    } // <--- coffees aur two_coffees dono yahi mar gaye!

    println!("Back to main: cities is still here!");
}

/*
EXPECTED OUTPUT:
First two: ["London", "New York"]
Coffees: ["Latte", "Mocha"]
Back to main: cities is still here!
*/

/*
🧠 Deep Feel:
Jab function return mein reference deta hai, toh wo 'Borrowed' data hota hai.
Rust kehta hai: 'Jisne borrow diya (Input), uska lifetime
borrow lene wale (Output) se bada ya barabar hona chahiye.'
Yaha compiler 'Lifetime Elision' rules use karke chup-chap
lifetimes ko connect kar deta hai. Hamein kuch likhne ki bhi zaroorat nahi padi!
*/

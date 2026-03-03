// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 5)
// ==============================

// Topic: Invalid Lifetimes I (The "Deserted Island" Problem)
// Feel: 
// "Bhai, agar aap ek scope `{}` ke andar ek variable banao, 
// aur uske marte hi uska 'Pata' (Address) bahar le jao...
// Toh bahar wala banda kahan jayega? Usse toh khali rasta milega! 
// Isse kehte hain 'Dangling Reference'."

fn main() {
    /* 🛑 YE CODE COMPILE NAHI HOGA
    let some_cities; 

    {
        let cities = vec![
            String::from("London"),
            String::from("New York"),
            String::from("Barcelona"),
        ];

        // Feel: 'cities' yaha zinda hai.
        some_cities = &cities[..2]; 
    } // <--- 'cities' yaha mar gaya (Memory freed!)

    // Error: some_cities ek mare hue variable ko point kar raha hai!
    // println!("{:?}", some_cities); 
    */

    println!("Bhai, compile error se bachne ke liye code comment kiya hai!");
    println!("Feel: Mezban (cities) mar gaya, toh Guest (some_cities) kahan thahrega?");
}

/*
EXPECTED OUTPUT:
Bhai, compile error se bachne ke liye code comment kiya hai!
Feel: Mezban (cities) mar gaya, toh Guest (some_cities) kahan thahrega?
*/

/*
🧠 Deep Feel:
`some_cities` ki lifetime tab tak honi chahiye jab tak reference valid hai.
Par reference jis cheez ka hai (`cities`), wo pehle hi khatam ho gayi.
Rust ka checker 'Borrow Checker' yaha check karta hai: 
`Scope of Data (Owner)` MUST BE LONGER THAN `Scope of Reference`.
Yaha Owner (cities) ka scope chota tha, isliye Error!
*/

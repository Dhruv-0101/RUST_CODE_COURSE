// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 11)
// ==============================

// Topic: Conditional Method Implementation (Trait Bounds on `impl`)
// Feel: 
// "Bhai, ye function sabke liye nahi hai. 
// Ye sirf un 'Structs' ke liye hai jinhone ek specific Trait implement kiya hai.
// VIP entry only!"

use std::fmt::Display;

struct Package<T> {
    content: T,
}

// 1. Generic Implementation (Sabke liye available hai)
impl<T> Package<T> {
    fn new(content: T) -> Self {
        Self { content }
    }
}

// 2. Conditional Implementation (Sirf tab jab T 'Display' implement karta ho)
// Feel: "Bhai, agar main content ko 'Display' (print) nahi kar sakta, toh 'print_content' method exist hi nahi karega!"
impl<T: Display> Package<T> {
    fn print_content(&self) {
        println!("📦 Package contains: {}", self.content);
    }
}

fn main() {
    // String 'Display' implement karta hai
    let p1 = Package::new("A shiny watch");
    p1.print_content(); // ✅ Ye chalega

    // i32 'Display' implement karta hai
    let p2 = Package::new(100);
    p2.print_content(); // ✅ Ye bhi chalega

    // Vec<i32> 'Display' implement NAHI karta (standard way mein)
    let p3 = Package::new(vec![1, 2, 3]);
    
    // UNCOMMENT NEXT LINE TO SEE COMPILER CRY:
    // p3.print_content(); 
    // Error: "method `print_content` not found for `Package<Vec<i32>>`"
    
    println!("Main logic finished safely!");
}

/*
EXPECTED OUTPUT:
📦 Package contains: A shiny watch
📦 Package contains: 100
Main logic finished safely!
*/

/*
🧠 Deep Feel:
Ye Rust ki bohot powerful feature hai. 
Aap ek generic struct banate ho, lekin uske methods 'Unlock' tabhi hote hain jab underlying type ke paas wo capabilities (traits) hon.
Isse 'Blanket Implementations' bhi kehte hain jab bade scale pe use ho.
*/


// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 21)
// ==============================

// Topic: Traits with Generics
// Feel: 
// "Bhai, Trait bhi Generic ho sakta hai! 
// Matlab ek hi Trait alag-alag types ke data ke saath kaam kar sakta hai. 
// Jaise 'Processor' jo kabhi Integer process kare, kabhi String."

trait Processor<T> {
    fn process(&self, input: T);
}

struct Printer;

// Case 1: Printer jo String process karta hai
impl Processor<&str> for Printer {
    fn process(&self, input: &str) {
        println!("🖨️ Printing text: {}", input);
    }
}

// Case 2: Printer jo Integer process karta hai (Same trait, alag T!)
impl Processor<i32> for Printer {
    fn process(&self, input: i32) {
        println!("🖨️ Printing number: {}", input);
    }
}

fn main() {
    let p = Printer;

    // Same object, same trait method, lekin alag types!
    p.process("Hello Rust!");
    p.process(404);
}

/*
EXPECTED OUTPUT:
🖨️ Printing text: Hello Rust!
🖨️ Printing number: 404
*/

/*
🧠 Deep Feel:
Generic Traits bohot powerful hain. 
Aap ek hi Trait ko multiple times implement kar sakte ho for the SAME type, 
bas unke generic parameters (T) alag hone chahiye. 
Ye 'Associated Types' se thoda alag hai (wo hum aage dekhenge).
Generics use tab karo jab aap chahte ho ki ek type multiple versions implement kare.
*/


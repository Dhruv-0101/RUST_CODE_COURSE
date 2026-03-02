// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 26)
// ==============================

// Topic: The Drop Trait (The Destructor)
// Feel: 
// "Bhai, Rust mein garbage collector nahi hota. 
// Par humare paas 'Drop' trait hai! 
// Ye ek aakhri (final) wish ki tarah hai: 'Jab main memory se jaaun, toh ye kaam kar dena' (like closing a file or network connection)."

struct DatabaseConnection {
    name: String,
}

impl Drop for DatabaseConnection {
    // drop method tab chalega jab variable scope se bahar jayega
    fn drop(&mut self) {
        // Feel: "Jate-jate light band karke ja raha hoon."
        println!("🗑️ Closing Database Connection: '{}'. Resources cleaned up!", self.name);
    }
}

fn main() {
    println!("--- Program Start ---");
    
    {
        let conn = DatabaseConnection { name: String::from("MainDB") };
        println!("Using connection: {}...", conn.name);
        
        // 'conn' yaha scope ke bahar chala jayega (End of block)
    } 

    println!("--- Program Continues ---");
    
    let _another_conn = DatabaseConnection { name: String::from("SecondaryDB") };
    
    println!("Program about to end...");
}

/*
EXPECTED OUTPUT:
--- Program Start ---
Using connection: MainDB...
🗑️ Closing Database Connection: 'MainDB'. Resources cleaned up!
--- Program Continues ---
Program about to end...
🗑️ Closing Database Connection: 'SecondaryDB'. Resources cleaned up!
*/

/*
🧠 Deep Feel:
Drop trait 'RAII' (Resource Acquisition Is Initialization) pattern ka heart hai.
Aapko manual memory free karne ki zaroorat nahi padti, Rust automatic 
'drop' call kar deta hai jab variable ka kaam khatam ho jata hai.
IMPORTANT: Aap drop ko manually call nahi kar sakte (`conn.drop()` is illegal). 
Agar jaldi drop karna hai toh `std::mem::drop(conn)` use karo.
*/
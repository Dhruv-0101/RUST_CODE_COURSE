// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 15)
// ==============================

// Topic: The 'static Lifetime (The Immortal)
// Feel: 
// "Bhai, `'static` matlab wo variable jo hamesha (Immortal) hai. 
// Ye program ke birth se death tak valid hai!
// Jaise String Literals `"Hello"` ya `const` values."

const VERSION: &str = "1.0.0";

// Feel: &'static str matlab: "Bhai, ye string puri zindagi zinda rahegi!"
fn get_greeting() -> &'static str {
    "Hello Programmers!"
}

fn get_version() -> &'static str {
    &VERSION
}

fn main() {
    // string literals are baked into the binary. Memory is allocated Forever!
    let hello = get_greeting();
    println!("Greeting: {}", hello);

    let v = get_version();
    println!("App Version: {}", v);
}

/*
EXPECTED OUTPUT:
Greeting: Hello Programmers!
App Version: 1.0.0
*/

/*
🧠 Deep Feel:
`'static` ek reserved keyword hai. 
Iska matlab hai ki data **Binary** mein store kiya gaya hai 
(jaise string literals ya static constants). 
Is memory ka address hamesha valid hota hai. 
Note: Kuch resources (like Box::leak) se hum runtime pe bhi `'static` 
reference bana sakte hain (Par uska apna darr aur power dono hai!). 
Asli feel: `'static` is the 'Golden Age' that never ends.
*/

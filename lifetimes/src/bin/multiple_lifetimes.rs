// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 16)
// ==============================

// Topic: Multiple Lifetimes ('a, 'b)
// Feel: 
// "Bhai, kabhi-kabhi do cheezein alag-alag scope mein zinda rehti hain. 
// Hum unhe ek hi `'a` lifetime se tie nahi kar sakte (Varna compiler sabse 
// choti wali lifetime ko sabpe force karega!). 
// Tab hum use karte hain multiple tags: `'a` aur `'b`!"

#[derive(Debug)]
struct TravelPlan<'a, 'b> {
    // Feel: from 'b' jitna zinda rahega, aur to 'a' jitna.
    from: &'b str,
    to: &'a str,
}

// Function with multiple lifetimes
fn get_starting_point<'a, 'b>(from: &'a str, _to: &'b str) -> &'a str {
    // Rust only cares about 'a (starting point) lifetime
    from 
}

fn main() {
    let city1 = String::from("Mumbai");
    let result;

    {
        let city2 = String::from("Delhi"); // inner scope
        
        // city1 lives until end of main. city2 lives only in block.
        // Multiple lifetimes let us return city1 even though city2 is about to die!
        result = get_starting_point(&city1, &city2);
        println!("Inner: result is pointing to {}", result);
    } 

    // result points to Mumbai, so it's still alive! 🎯
    println!("Outer: result is still valid: {}", result);

    println!("Bhai, dhyan dena: Multiple lifetimes are useful for independent borrow scopes.");

}

/*
EXPECTED OUTPUT:
Inner: result is pointing to Mumbai
Outer: result is still valid: Mumbai
Bhai, dhyan dena: Multiple lifetimes are useful for independent borrow scopes.
*/

/*
🧠 Deep Feel:
Multiple lifetimes (`<'a, 'b>`) let you be more flexible. 
If you use only `'a` for everything, Rust will pick the **SHORTED** scope 
among all inputs and apply it to the output. 
With `'a` and `'b`, you tell the compiler: 'Bhai, ye dono independent hain. 
Mera return value sirf s1 (`'a`) se connected hai, toh ignore whatever happens to s2 (`'b`).'
Feel: Power of Precision! 🤖🚀
*/

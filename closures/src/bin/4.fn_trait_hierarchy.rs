// ==============================
// RUST CLOSURES — THE TRAIT HIERARCHY (EPISODE 7.5)
// ==============================

// Topic: The Fn, FnMut, and FnOnce Hierarchy
// Feel:
// "Bhai, ye Rust ka 'Hierarchy of Power' hai.
// Closures ke 3 traits hote hain, aur ye ek dusre ke andar 'Samaaye' (Nested) hote hain.
// Isse aise samjho: 'Shareef' (Fn) -> 'Worker' (FnMut) -> 'Greedy' (FnOnce)."

fn main() {
    println!("--- The Closure Hierarchy Journey ---");

    // ---------------------------------------------------------
    // 1. Fn (The Shareef Reader 📖)
    // ---------------------------------------------------------
    let name = String::from("Rust");
    let say_hello = || println!("Hello, {}", name);
    take_fn(say_hello);

    // ---------------------------------------------------------
    // 2. FnMut (The Worker Painter 🎨)
    // ---------------------------------------------------------
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Counter is now: {}", counter);
    };
    take_fn_mut(increment);

    // ---------------------------------------------------------
    // 3. FnOnce (The Greedy Eater 🍔)
    // ---------------------------------------------------------
    let treasure = String::from("Gold");
    let eat_treasure = || {
        let stolen = treasure;
        println!("I stole the {}", stolen);
    };
    take_fn_once(eat_treasure);

    println!("\nBhai, Hierarchy ka chart niche comments mein dekho!");
}

// ==========================================
// 🧠 SYNTAX POST-MORTEM (Bhai, yaha dhyan do!)
// ==========================================

// 1. WHY THE `<F>`?
// Feel: Har closure ka type unique hota hai (Anonymous).
// Isliye hum function ko bolte hain: "Bhai, mujhe nahi pata closure ka asli naam kya hai,
// bas use 'F' maan lo." (F is a Generic Placeholder).

// 2. WHY THE `where` CLAUSE?
// Feel: Ye ek 'Filtering Machine' hai. Ye kehta hai: "F kuch bhi ho sakta hai,
// LEKIN... use ye 'Trait' (Shart) poori karni padegi."

// ---------------------------------------------------------
// Case A: Strict Reader (Fn)
// ---------------------------------------------------------
fn take_fn<F>(f: F)
where
    F: Fn(), // Shart: "Bhai, sirf read-only closures andar aayenge."
{
    f(); // Use it!
}

// ---------------------------------------------------------
// Case B: Worker Painter (FnMut)
// ---------------------------------------------------------
fn take_fn_mut<F>(mut f: F)
// Note: 'mut f' zaroori hai badalne ke liye
where
    F: FnMut(), // Shart: "Bhai, jo data badalte hain, wo closures allowed hain."
{
    f();
}

// ---------------------------------------------------------
// Case C: One-Time Eater (FnOnce)
// ---------------------------------------------------------
fn take_fn_once<F>(f: F)
where
    F: FnOnce(), // Shart: "Bhai, jo ownership kha jate hain, wo bhi allowed hain."
{
    f();
}

/*
🧠 DEEP HIERARCHY CHART:

     [   FnOnce   ]  <- Sabse Bada Circle (Requirement: At least once)
          ↑
     [    FnMut   ]  <- Beech wala (Requirement: Can change data)
          ↑
     [     Fn     ]  <- Sabse Chota (Requirement: Read-Only)

Relationship Logic:
- Jo 'Fn' hai, wo automatically 'FnMut' bhi hai aur 'FnOnce' bhi.
- Jo 'FnMut' hai, wo 'FnOnce' toh hai, par 'Fn' nahi hai.
- Jo 'FnOnce' hai, wo bas akela hai (One-Timer).
*/

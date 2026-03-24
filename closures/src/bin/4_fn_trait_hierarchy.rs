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
    // println!("{} is stolen", treasure); //error

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

---
Bhai, ye Rust mein Closures (anonymous functions) ke kaam karne ka tareeka hai.
Rust ko ye pata hona chahiye ki koi closure apne variables ke saath kya kar raha hai —
kya wo unhe bas read kar raha hai, change kar raha hai, ya poora ka poora consume (move) kar raha hai.

Isi ke liye ye 3 traits hote hain (Hierarchy wise):

1. Fn (The Reader 📖)
   Kaam: Ye sirf variables ko "read" karta hai.
   Analogy: Ek Library ki book — aap sirf padh sakte ho, usme pencil nahi chala sakte.
   Property: Isse aap jitni baar chaho utni baar call kar sakte ho.
   Code Example (line 18): let say_hello = || println!("Hello, {}", name); (Sirf name ko print kar raha hai).

2. FnMut (The Worker Painter 🎨)
   Kaam: Ye variables ko "mutate" (Badal) sakta hai.
   Analogy: Ek notebook — aap isme likh sakte ho, purana mita ke naya likh sakte ho.
   Property: Isse call karne ke liye closure khud 'mut' hona chahiye.
   Code Example (line 25): counter += 1; (Variable ki value change kar raha hai).

3. FnOnce (The Greedy Eater 🍔)
   Kaam: Ye variables ki ownership "le leta hai" (Move).
   Analogy: Ek Burger — ek baar kha liya toh khatam, dobara nahi call kar sakte.
   Property: Ye sirf ek hi baar run ho sakta hai kyunki ye data ko consume kar leta hai.
   Code Example (line 35): let stolen = treasure; (Ownership treasure se stolen mein chali gayi).

---
Hierarchy (Asli Khel 🧠)
Inka aapas mein ek rishta hota hai:

- Fn is a subtype of FnMut
- FnMut is a subtype of FnOnce

Iska matlab:
- Jo closure Fn hai (sirf read), wo automatically FnMut aur FnOnce bhi hai.
- Lekin jo FnOnce hai (data kha gaya), wo Fn nahi ban sakta.

| Trait   | Captured Data            | How many times? |
| :------ | :-----------             | :-------------- |
| Fn      | Immutable Reference (&T) | Many            |
| FnMut   | Mutable Reference (&mut T)| Many            |
| FnOnce  | Value (T)                | Only Once       |

Aapke upar ke chart mein ye circle aur arrows isi logic ko dikha rahe hain!
*/

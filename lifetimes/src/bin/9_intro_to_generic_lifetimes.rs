// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 9)
// ==============================

// Topic: Generic Lifetime Annotations ('a) - The Security Guard Contract
//
// Feel:
// "Bhai, Generics ('a) koi magic nahi hai. Ye ek 'Security Contract' hai.
// Jab hum function ko do references dete hain, compiler darta hai:
// 'Agar ek variable jaldi mar gaya aur humne uska reference return kar diya, toh crash ho jayega!'
//
// Tab hum `'a` likh kar bolte hain:
// 'Suno Guard saab (Compiler), ye s1, s2 aur return value TEENO ek hi contract ('a) ke under hain.
// Return value tab tak hi valid rahegi jab tak s1 aur s2 DONO zinda hain.'"

// Contract: 'a is the SHORTEST life among the inputs.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // Iska matlab: 'Return value s1 ya s2, kisi se bhi aaye,
    // uska expiration date wahi hoga jo s1 aur s2 mein se pehle expire hoga!'
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn main() {
    // SCENARIO 1: Everything is cool (Donon lambi race ke ghode hain)
    let city1 = String::from("London");
    let city2 = "New York";

    let result = longest(&city1, city2);
    println!("1. Longest city (Safe): {}", result);

    // SCENARIO 2: The "Dhadkan" Moment (One expires early)
    let result_storage;
    let city_long = String::from("Barcelona"); // Lambi lifetime

    {
        let city_short = String::from("Goa"); // Choti lifetime (Isi scope tak)

        // Humne contract sign kiya: result_storage ki validity = Shortest between city_long & city_short.
        // Yani result_storage sirf is bracket } tak hi valid hai!
        result_storage = longest(city_long.as_str(), city_short.as_str());

        println!("2. Inside Scope (Still Safe): {}", result_storage);
    } // <--- 'Goa' (city_short) yahan MAR GAYA! 💀

    // println!("3. Outside Scope: {}", result_storage);
    // ❌ ERROR! Agar aap upar wali line uncomment karoge, Rust chilayega:
    // "Bhai! result_storage mein 'Goa' ka reference ho sakta hai, aur Goa toh mar gaya!"
    // "Tumne contract ('a) toda hai!"

    println!(
        "3. Main: result_storage ko bahar use karna mana hai, kyunki contract expire ho gaya."
    );
}

/*
🧠 Deep Feel Summary:
1. `'a` kisi ki umar badhata nahi hai (it doesn't extend life).
2. Ye bas variables ke beech ek RISHTA (Relationship) batata hai.
3. Compiler hamesha SABSE CHOTI lifetime ko `'a` manta hai safety ke liye.
4. Input zinda -> Output zinda. Input mara -> Output INVALID. Simple!
*/

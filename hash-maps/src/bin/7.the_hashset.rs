use std::collections::HashSet;

/*
  🧠 CONCEPTUAL FEEL: "HashSet vs HashMap"
  -----------------------------------------
  1. HashSet sirf UNIQUE KEYS se banta hai. Isme "Values" ki koi jagah nahi hoti! 🚫
  2. HashMap mein hum (Key -> Value) pairs store karte hain, lekin HashSet mein
     humein sirf ye jaanna hota hai ki "Kya ye item list mein hai ya nahi?"

  Technically: Rust internally HashSet ko ek HashMap hi manta hai, jisme
  Values wali side khali (empty tuple `()`) hoti hai! 🤯
*/

fn main() {
    // 🎨 HashSet ko ek "VIP Guest List" ki tarah imagine karo.
    // Isme ek hi naam do baar nahi likha ja sakta! ✨
    let mut concert_queue: HashSet<&str> = HashSet::new();
    println!("{:?}", concert_queue); // Khali list: {}

    // 📍 Inserting elements.
    // HashSet ka koi fixed order nahi hota.
    // Megan pehle aa sakti hai, ya Molly—isme sequence matter nahi karta!
    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    println!("{:?}", concert_queue); // Output: {"Megan", "Molly"} (kisi bhi order mein)
    println!("{}", concert_queue.len()); // Count: 2

    // ⛔ UNIQUE FEEL:
    // Agar main "Molly" ko wapas insert karu...
    concert_queue.insert("Molly");
    // ...toh HashSet use ignore kar dega, kyunki wo pehle se list mein hai! ✅
    println!("{:?}", concert_queue); // Output: Abhi bhi 2 hi elements hain {"Megan", "Molly"}

    // 🔪 REMOVING:
    // .remove() return karta hai 'true' agar element mila, 'false' agar nahi mila.
    println!("{}", concert_queue.remove("Megan")); // true (Megan list se bahar!)
    println!("{}", concert_queue.remove("Franny")); // false (Franny toh list mein thi hi nahi!)
    println!("{:?}", concert_queue); // Output: {"Molly"}

    // ⚡ MEMBERSHIP TESTING (Super Fast!):
    // List mein koi hai ya nahi check karna HashSet mein bahut fast hota hai (O(1)).
    println!("{}", concert_queue.contains("Molly")); // true (Molly is here!)
    println!("{}", concert_queue.contains("Fred")); // false

    // 💎 .get() returns an Option reference.
    // Iska use tab karte hain jab humein element ka access chahiye.
    println!("{:?}", concert_queue.get("Molly")); // Some("Molly")
    println!("{:?}", concert_queue.get("Joe")); // None
}

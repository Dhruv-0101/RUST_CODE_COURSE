// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 10)
// ==============================

// Topic: Returning Traits from Functions (The "Aha!" Moment)
// Feel:
// "Bhai, ye 'impl Trait' return karna ek 'Black Box' return karne jaisa hai.
// Aap caller ko 'Remote' de rahe ho, unhe ye nahi bata rahe ki andar 'Sony' ka circuit hai ya 'LG' ka.
// Caller ko bas 'Button' dabane se matlab hai!"

trait RemoteControl {
    fn press_power_button(&self) -> String;
}

struct SonyRemote;
impl RemoteControl for SonyRemote {
    fn press_power_button(&self) -> String {
        String::from("📺 Sony TV is now ON!")
    }
}

struct LGRemote;
impl RemoteControl for LGRemote {
    fn press_power_button(&self) -> String {
        String::from("📺 LG TV is now ON!")
    }
}

// ------------------------------
// THE "AHA!" FACTORY 🏭
// ------------------------------
// Feel: "Main 'impl RemoteControl' return karunga. Caller ko 'SonyRemote' naam
// dikhne ki zaroorat nahi hai. Main asiliyat chhupa (Hide) raha hoon!"
fn get_my_remote() -> impl RemoteControl {
    // Note: Rust mein 'impl Trait' ke saath ek function se
    // sirf EK HI concrete type return ho sakta hai.
    // Mix nahi kar sakte (Sony aur LG ek saath nahi).
    SonyRemote
}

fn main() {
    println!("--- The Remote Factory ---");

    // 'my_remote' ka type 'impl RemoteControl' hai.
    // Humne uski 'asli identity' (SonyRemote) mask kar di hai.
    let my_remote = get_my_remote();

    // Caller ko bas isse matlab hai ki behavior (method) chal raha hai!
    println!("Result: {}", my_remote.press_power_button());
}

/*
EXPECTED OUTPUT:
--- The Remote Factory ---
Result: 📺 Sony TV is now ON!
*/

/*
🧠 DEEP FEEL (Simplest Explanation):

1. Abstraction (Parda 🎭):
   "Bhai, jab main '-> impl RemoteControl' likhta hoon, toh main caller se kehta hoon:
   'Tujhe kya lena-dena ki andar Sony hai ya LG? Tu bas button daba aur TV chalu kar!' 
   Caller asli type pe depend nahi hota, sirf behavior pe hota hai."

2. Privacy (Chhupana 🤫):
   "Agar main SonyRemote return kar raha hoon, toh caller shayad Sony-specific
   cheezein use karne ki koshish kare. 'impl Trait' unhe allow hi nahi karta! 
   Wo sirf wahi kar sakte hain jo RemoteControl trait mein hai."

3. The "One Concrete Type" Rule:
   "Yaad rakhna! 'impl Trait' return ka matlab ye nahi hai ki aap dynamic ho gaye ho.
   Compiler ko compile time pe fix karna hi padega ki andar Sony hai ya LG. 
   Ek baar fix ho gaya, toh wo wahi rahega."

Moral: 'impl Trait' in return is for Hiding Information and keeping the API Clean.
Ab aaya na simple 'Aha!' moment? 🦀✨
*/

// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 4)
// ==============================

// Topic: Non-Lexical Lifetimes (NLL) - The Smart Compiler
// Feel: 
// "Bhai, pehle Rust mein 'Zid' thi: 'Agar reference braces `{}` ke andar hai, 
// toh wo marte dam tak (brace ke end tak) zinda rahega'. 
// Par NLL ke baad Rust smart ho gaya: 'Jaha kaam khatam, waha lifetime khatam!'"

fn main() {
    // 🖊️ ANALOGY: 
    // "Socho aapne apne dost (Owner) se ek Pen (Data) mangi.
    // Dost: 'Bhai, jab tak tere paas pen hai, main usse touch nahi karunga.'
    
    let mut pen_data = String::from("Hi"); 

    // 1. Aapne pen borrow ki (Mutable Reference starts)
    let my_borrow = &mut pen_data; 
    
    // 2. Aapne pen se likha (Use of reference)
    my_borrow.push_str(" Friends"); 

    // --- AHA! MOMENT STARTS HERE ---
    
    // Feel: Purane Rust (pre-2018) mein 'my_borrow' abhi bhi zinda mana jata 
    // jab tak ye `main` function ka `}` nahi aata. 
    // Isliye niche wali line pe ERROR aata kyunki owner (dost) pen touch kar raha hai.
    
    println!("Dost says: Main pen wapas le raha hun!");
    pen_data.push('!'); // ✅ SUCCESS in Modern Rust! (NLL magic)
    
    // Kyun? Kyunki compiler ne dekha: 'my_borrow' ka aakhri use 
    // line 20 pe ho gaya. Uske baad wo bekaar hai. 
    // Toh compiler ne borrow ko line 20 ke baad hi 'khatam' kar diya!

    println!("Final Result: {}", pen_data);

    /* 
    ⚠️ LEKIN AGAR:
    Agar main yaha 'println!("{}", my_borrow)' likhu, 
    toh fir se ERROR aa jayega! 
    Kyunki compiler bolega: 'Bhai, tune toh kaha tha kaam khatam, 
    par tu line 35 pe fir se mang raha hai. Toh line 27 (owner push) galat ho gayi!'
    */
}

/*
EXPECTED OUTPUT:
Dost says: Main pen wapas le raha hun!
Final Result: Hi Friends!
*/

/*
🧠 Deep Feel:
NLL matlab "Use-based Lifetimes" bajay "Scope-based Lifetimes".
- Purana Rust: 'Braces decide lifetimes' (Lexical).
- Naya Rust: 'Actual usage decides lifetimes' (Non-Lexical).

Moral: Rust hamesha koshish karta hai ki aapka code compile ho jaye, 
jab tak wo 'Memory Safe' hai. NLL usi safety aur flexibility ka combo hai! 🦾✨
*/

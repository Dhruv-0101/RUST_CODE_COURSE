fn main() {
    // 🍔 Hamare paas sauces ka ek "Vector" (list) hai.
    // Socho ye ek Stack of Plates hai - jo sabse upar (aakhri) hai, wo pehle niklega!
    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];

    /* 
    =========================================================
    🔄 THE "WHILE-LET" VIBE: (The Exhaustive Loop)
    =========================================================
    'while let' ka seedha matlab hai: 
    "Jab tak ye pattern match ho raha hai, tab tak loop chalate raho!"
    
    1. 'sauces.pop()' -> Ek-ek karke item nikalta hai (Ranch -> Ketchup -> Mayo).
    2. 'Some(sauce)'  -> Agar item mila (Some), toh 'sauce' mein value daalo aur loop chalao.
    3. 'None'          -> Jaise hi list khaali hui, loop apne aap 'break' ho jayega!
    =========================================================
    */

    while let Some(sauce) = sauces.pop() {
        // ✨ MAGIC: 'sauce' variable ab loop ke andar full power mein hai!
        println!("The next sauce is {sauce}");
    }

    // 🏆 RESULT: Yahan tak aate-aate loop 3 baar chal chuka hai aur list khaali!
}

/*
=========================================================
🍕 ANALOGY: The Pizza Box (The Hungry Friend)
=========================================================
- Aapne Pizza Box khola.
- 'while let Some(slice) = pizza.take_slice()'
- 🍕 "Ek slice uthaya" (Some). Kha liya. Loop chala.
- 🍕 "Doosra slice uthaya" (Some). Kha liya. Loop chala.
- 🚫 "Box check kiya..." (None). Ab kuch nahi hai. Loop khatam!

Pehla slice 'Ranch' kyun nikla? 
Kyunki `.pop()` hamesha "Aakhri" (Last) element ko pehle nikalta hai!
=========================================================
*/
